use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use std::fmt::Debug;
use tracing::instrument;

pub struct ComponentCaller<
    ChildCpState: Clone + Default + Debug + PartialEq,
    ChildCpEvent,
    Props,
    ReturnedTree: Element<ChildCpState, ChildCpEvent>,
    Comp: Fn(&ChildCpState, Props) -> ReturnedTree,
    ParentCpState = (),
    ParentCpEvent = NoEvent,
> {
    pub component: Comp,
    pub props: Props,
    pub _parent_state: std::marker::PhantomData<ParentCpState>,
    pub _parent_event: std::marker::PhantomData<ParentCpEvent>,
    pub _child_state: std::marker::PhantomData<ChildCpState>,
    pub _child_event: std::marker::PhantomData<ChildCpEvent>,
    pub _returned_tree: std::marker::PhantomData<ReturnedTree>,
}

#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = "Child: Default"))]
pub struct ComponentCallerData<
    ChildCpState: Clone + Default + Debug + PartialEq,
    ChildCpEvent,
    Child: VirtualDom<ChildCpState, ChildCpEvent>,
    ParentCpState,
    ParentCpEvent,
>(
    Child,
    std::marker::PhantomData<ParentCpState>,
    std::marker::PhantomData<ParentCpEvent>,
    std::marker::PhantomData<ChildCpState>,
    std::marker::PhantomData<ChildCpEvent>,
);

impl<
        ParentCpState,
        ParentCpEvent,
        ChildCpState: Clone + Default + Debug + PartialEq,
        ChildCpEvent,
        Props,
        ReturnedTree: Element<ChildCpState, ChildCpEvent>,
        Comp: Fn(&ChildCpState, Props) -> ReturnedTree,
    >
    ComponentCaller<
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
        ComponentCaller {
            component,
            props,
            _parent_state: Default::default(),
            _parent_event: Default::default(),
            _child_state: Default::default(),
            _child_event: Default::default(),
            _returned_tree: Default::default(),
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
        Comp: Fn(&ChildCpState, Props) -> ReturnedTree,
    > std::fmt::Debug
    for ComponentCaller<
        ChildCpState,
        ChildCpEvent,
        Props,
        ReturnedTree,
        Comp,
        ParentCpState,
        ParentCpEvent,
    >
{
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
        Comp: Fn(&ChildCpState, Props) -> ReturnedTree,
    > Element<ParentCpState, ParentCpEvent>
    for ComponentCaller<
        ChildCpState,
        ChildCpEvent,
        Props,
        ReturnedTree,
        Comp,
        ParentCpState,
        ParentCpEvent,
    >
{
    type Event = ChildCpEvent;
    type AggregateChildrenState = (ChildCpState, ReturnedTree::AggregateChildrenState);
    type BuildOutput = ComponentCallerData<
        ChildCpState,
        ChildCpEvent,
        ReturnedTree::BuildOutput,
        ParentCpState,
        ParentCpEvent,
    >;

    #[instrument(name = "Component", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let element_tree = (self.component)(&prev_state.0, self.props);
        let (element, component_state) = element_tree.build(prev_state.1);
        (
            ComponentCallerData(
                element,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ),
            (prev_state.0, component_state),
        )
    }
}

impl<
        ParentCpState,
        ParentCpEvent,
        ChildCpState: Clone + Default + Debug + PartialEq,
        ChildCpEvent,
        Child: VirtualDom<ChildCpState, ChildCpEvent>,
    > VirtualDom<ParentCpState, ParentCpEvent>
    for ComponentCallerData<ChildCpState, ChildCpEvent, Child, ParentCpState, ParentCpEvent>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element_tree::assign_empty_state_type;
    use crate::element_tree::ElementExt;
    use crate::element_tree::NoEvent;
    use crate::elements::{Button, Label};
    use crate::make_row;

    use insta::assert_debug_snapshot;
    use test_env_log::test;

    // TODO - add tracing, and detect when this function is called by tests
    fn my_component(state: &u16, props: i64) -> impl Element<u16, NoEvent> {
        make_row!(
            Button::new("Press me").map_event(|state: &mut u16, _| {
                *state += 1;
                None
            }),
            Label::new(format!("Values: {} {}", state, props)),
        )
    }

    #[test]
    fn call_component() {
        let comp_caller = ComponentCaller::prepare(my_component, 16);
        assign_empty_state_type(&comp_caller);

        let prev_state = (999, Default::default());
        let (component_result, component_state) = comp_caller.build(prev_state);

        assert_eq!(component_state.0, 999);
        assert_debug_snapshot!(component_result);

        // TODO - process_event
    }

    // TODO
    // - Widget test
    // - Events
}
