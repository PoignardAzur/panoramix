use crate::element_tree::{CompCtx, Element, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use std::fmt::Debug;
use tracing::instrument;

pub trait Component<ParentCpState = (), ParentCpEvent = NoEvent>: Debug {
    type LocalState: Clone + Default + Debug + PartialEq;
    type LocalEvent;
    type Output: Element<Self::LocalState, Self::LocalEvent>;

    fn call(self, ctx: &CompCtx) -> Self::Output;
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ComponentHolder<T>(pub T);

#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = "Child: Default"))]
pub struct ComponentOutput<
    ChildCpState: Clone + Default + Debug + PartialEq,
    ChildCpEvent,
    Child: VirtualDom<ChildCpState, ChildCpEvent>,
    ParentCpState,
    ParentCpEvent,
>(
    Child,
    std::marker::PhantomData<(ParentCpState, ParentCpEvent, ChildCpState, ChildCpEvent)>,
);

impl<
        LocalState: Clone + Default + Debug + PartialEq + 'static,
        LocalEvent,
        ParentCpState,
        ParentCpEvent,
        T: Debug,
    > Element<ParentCpState, ParentCpEvent> for ComponentHolder<T>
where
    T: Component<ParentCpState, ParentCpEvent, LocalState = LocalState, LocalEvent = LocalEvent>,
{
    type Event = LocalEvent;
    type AggregateChildrenState = (
        LocalState,
        <T::Output as Element<LocalState, LocalEvent>>::AggregateChildrenState,
    );
    type BuildOutput = ComponentOutput<
        LocalState,
        LocalEvent,
        <T::Output as Element<LocalState, LocalEvent>>::BuildOutput,
        ParentCpState,
        ParentCpEvent,
    >;

    #[instrument(name = "Component", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        // FIXME - clone
        let ctx = CompCtx {
            local_state: Box::new(prev_state.0.clone()),
        };
        let element_tree = self.0.call(&ctx);
        let (element, component_state) = element_tree.build(prev_state.1);
        (
            ComponentOutput(element, Default::default()),
            (prev_state.0, component_state),
        )
    }
}

impl<
        ChildCpState: Clone + Default + Debug + PartialEq,
        ChildCpEvent,
        Child: VirtualDom<ChildCpState, ChildCpEvent>,
        ParentCpState,
        ParentCpEvent,
    > VirtualDom<ParentCpState, ParentCpEvent>
    for ComponentOutput<ChildCpState, ChildCpEvent, Child, ParentCpState, ParentCpEvent>
{
    type Event = ChildCpEvent;
    type AggregateChildrenState = (ChildCpState, Child::AggregateChildrenState);
    type TargetWidgetSeq = Child::TargetWidgetSeq;

    #[instrument(name = "Component", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        self.0.update_value(other.0);
    }

    #[instrument(name = "Component", skip(self))]
    fn init_tree(&self) -> Child::TargetWidgetSeq {
        self.0.init_tree()
    }

    #[instrument(name = "Component", skip(self, other, widget_seq, ctx))]
    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Child::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.0.reconcile(&other.0, widget_seq, ctx);
    }

    #[instrument(
        name = "Component",
        skip(self, _component_state, children_state, widget_seq, cx)
    )]
    fn process_local_event(
        &self,
        _component_state: &mut ParentCpState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Child::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        self.0
            .process_event(&mut children_state.0, &mut children_state.1, widget_seq, cx)
    }
}

// ---

pub struct ComponentCaller2<
    ChildCpState: Clone + Default + Debug + PartialEq,
    ChildCpEvent,
    Props,
    ReturnedTree: Element<ChildCpState, ChildCpEvent>,
    Comp: Fn(&CompCtx, Props) -> ReturnedTree,
    ParentCpState = (),
    ParentCpEvent = NoEvent,
> {
    pub component: Comp,
    pub props: Props,
    pub _markers: (
        std::marker::PhantomData<ParentCpState>,
        std::marker::PhantomData<ParentCpEvent>,
        std::marker::PhantomData<ChildCpState>,
        std::marker::PhantomData<ChildCpEvent>,
        std::marker::PhantomData<ReturnedTree>,
    ),
}

impl<
        ParentCpState,
        ParentCpEvent,
        ChildCpState: Clone + Default + Debug + PartialEq,
        ChildCpEvent,
        Props,
        ReturnedTree: Element<ChildCpState, ChildCpEvent>,
        Comp: Fn(&CompCtx, Props) -> ReturnedTree,
    >
    ComponentCaller2<
        ChildCpState,
        ChildCpEvent,
        Props,
        ReturnedTree,
        Comp,
        ParentCpState,
        ParentCpEvent,
    >
{
    pub fn prepare(component: Comp, props: Props) -> Self {
        ComponentCaller2 {
            component,
            props,
            _markers: Default::default(),
        }
    }
}

impl<
        ParentCpState,
        ParentCpEvent,
        ChildCpState: Clone + Default + Debug + PartialEq,
        ChildCpEvent,
        Props,
        ReturnedTree: Element<ChildCpState, ChildCpEvent>,
        Comp: Fn(&CompCtx, Props) -> ReturnedTree,
    > std::fmt::Debug
    for ComponentCaller2<
        ChildCpState,
        ChildCpEvent,
        Props,
        ReturnedTree,
        Comp,
        ParentCpState,
        ParentCpEvent,
    >
{
    // TODO
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentCaller")
            .field("component", &std::any::type_name::<Comp>())
            .field("props", &"<props>")
            .finish()
    }
}

impl<
        ParentCpState,
        ParentCpEvent,
        ChildCpState: Clone + Default + Debug + PartialEq,
        ChildCpEvent,
        Props,
        ReturnedTree: Element<ChildCpState, ChildCpEvent>,
        Comp: Fn(&CompCtx, Props) -> ReturnedTree,
    > Component<ParentCpState, ParentCpEvent>
    for ComponentCaller2<
        ChildCpState,
        ChildCpEvent,
        Props,
        ReturnedTree,
        Comp,
        ParentCpState,
        ParentCpEvent,
    >
{
    type LocalState = ChildCpState;
    type LocalEvent = ChildCpEvent;
    type Output = ReturnedTree;

    fn call(self, ctx: &CompCtx) -> Self::Output {
        (self.component)(&ctx, self.props)
    }
}
