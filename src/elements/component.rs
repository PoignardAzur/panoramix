use crate::element_tree::{CompCtx, Element, VirtualDom};
use crate::glue::GlobalEventCx;

use crate::element_tree::ReconcileCtx;

use crate::elements::any_element::{AnyStateBox, ElementBox, VirtualDomBox};

use derivative::Derivative;
use std::fmt::Debug;

pub trait Component: Debug + Clone {
    type Props: Clone + Default + Debug + PartialEq + 'static;
    type LocalEvent: 'static;
    type LocalState: Clone + Default + Debug + PartialEq + 'static;

    fn new<ParentCpEvent, ParentCpState>(
        props: Self::Props,
    ) -> ComponentHolder<Self, ParentCpEvent, ParentCpState>;

    fn name() -> &'static str;

    #[doc(hidden)]
    fn call_indirect<ParentCpEvent, ParentCpState>(
        &self,
        prev_state: (Self::LocalState, Option<AnyStateBox>),
        props: Self::Props,
    ) -> (
        ComponentOutput<
            Self::LocalEvent,
            Self::LocalState,
            VirtualDomBox<Self::LocalEvent, Self::LocalState>,
            ParentCpEvent,
            ParentCpState,
        >,
        (Self::LocalState, Option<AnyStateBox>),
    );
}

#[derive(Derivative, Default, PartialEq, Eq, Hash)]
#[derivative(Clone(bound = "Comp::Props: Clone"))]
pub struct ComponentHolder<Comp: Component, ParentCpEvent, ParentCpState> {
    component: Comp,
    props: Comp::Props,
    _marker: std::marker::PhantomData<(ParentCpEvent, ParentCpState)>,
}

#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Default(bound = "Child: Default"))]
pub struct ComponentOutput<
    ChildCpEvent,
    ChildCpState: Clone + Default + Debug + PartialEq,
    Child: VirtualDom<ChildCpEvent, ChildCpState>,
    ParentCpEvent,
    ParentCpState,
> {
    child: Child,
    name: &'static str,
    _markers: std::marker::PhantomData<(ParentCpEvent, ParentCpState, ChildCpEvent, ChildCpState)>,
}

// ---

impl<Comp: Component, ParentCpEvent, ParentCpState>
    ComponentHolder<Comp, ParentCpEvent, ParentCpState>
{
    pub fn new(component: Comp, props: Comp::Props) -> Self {
        Self {
            component,
            props,
            _marker: Default::default(),
        }
    }
}

impl<Comp: Component, ParentCpEvent, ParentCpState> std::fmt::Debug
    for ComponentHolder<Comp, ParentCpEvent, ParentCpState>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(Comp::name()).field(&self.props).finish()
    }
}

impl<
        ChildCpEvent,
        ChildCpState: Clone + Default + Debug + PartialEq,
        Child: VirtualDom<ChildCpEvent, ChildCpState>,
        ParentCpEvent,
        ParentCpState,
    > std::fmt::Debug
    for ComponentOutput<ChildCpEvent, ChildCpState, Child, ParentCpEvent, ParentCpState>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(self.name).field(&self.child).finish()
    }
}

// ---

impl<Comp: Component, ParentCpEvent, ParentCpState>
    ComponentHolder<Comp, ParentCpEvent, ParentCpState>
{
    pub fn build_with<ReturnedTree: Element<Comp::LocalEvent, Comp::LocalState> + 'static>(
        _comp: Comp,
        comp_fn: impl Fn(&CompCtx, Comp::Props) -> ReturnedTree,
        prev_state: (Comp::LocalState, Option<AnyStateBox>),
        props: Comp::Props,
    ) -> (
        ComponentOutput<
            Comp::LocalEvent,
            Comp::LocalState,
            VirtualDomBox<Comp::LocalEvent, Comp::LocalState>,
            ParentCpEvent,
            ParentCpState,
        >,
        (Comp::LocalState, Option<AnyStateBox>),
    ) {
        let (local_state, children_state) = prev_state;
        let ctx = CompCtx {
            local_state: &local_state,
        };

        let element_tree = ElementBox::new(comp_fn(&ctx, props));
        let (vdom, new_children_state) = element_tree.build(children_state);

        (
            ComponentOutput {
                child: vdom,
                name: Comp::name(),
                _markers: Default::default(),
            },
            (local_state, new_children_state),
        )
    }
}

impl<Comp: Component, ParentCpEvent, ParentCpState> Element<ParentCpEvent, ParentCpState>
    for ComponentHolder<Comp, ParentCpEvent, ParentCpState>
{
    type Event = Comp::LocalEvent;
    type AggregateChildrenState = (Comp::LocalState, Option<AnyStateBox>);
    type BuildOutput = ComponentOutput<
        Comp::LocalEvent,
        Comp::LocalState,
        VirtualDomBox<Comp::LocalEvent, Comp::LocalState>,
        ParentCpEvent,
        ParentCpState,
    >;

    // TODO - add spans
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        self.component.call_indirect(prev_state, self.props)
    }
}

/// ---

impl<
        ChildCpEvent,
        ChildCpState: Clone + Default + Debug + PartialEq,
        Child: VirtualDom<ChildCpEvent, ChildCpState>,
        ParentCpEvent,
        ParentCpState,
    > VirtualDom<ParentCpEvent, ParentCpState>
    for ComponentOutput<ChildCpEvent, ChildCpState, Child, ParentCpEvent, ParentCpState>
{
    type Event = ChildCpEvent;
    type AggregateChildrenState = (ChildCpState, Child::AggregateChildrenState);
    type TargetWidgetSeq = Child::TargetWidgetSeq;

    // TODO - add spans
    fn init_tree(&self) -> Child::TargetWidgetSeq {
        self.child.init_tree()
    }

    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Child::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.child.reconcile(&other.child, widget_seq, ctx);
    }

    fn process_local_event(
        &self,
        _component_state: &mut ParentCpState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Child::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        self.child
            .process_event(&mut children_state.0, &mut children_state.1, widget_seq, cx)
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use crate as panoramix;

    #[derive(Debug, Default, Clone, PartialEq, Hash)]
    struct MyComponent;

    type MyPropsType = ();
    type MyLocalEvent = panoramix::NoEvent;
    type MyLocalState = u16;

    impl MyComponent {
        fn new<ParentCpEvent, ParentCpState>(
            props: MyPropsType,
        ) -> impl panoramix::Element<ParentCpEvent, ParentCpState> {
            <Self as panoramix::elements::component::Component>::new(props)
        }

        fn render(
            _ctx: &panoramix::CompCtx,
            _my_props: MyPropsType,
        ) -> impl panoramix::Element<MyLocalEvent, MyLocalState> {
            panoramix::elements::EmptyElement::new()
        }
    }

    impl panoramix::elements::component::Component for MyComponent {
        type Props = MyPropsType;
        type LocalState = MyLocalState;
        type LocalEvent = MyLocalEvent;

        fn new<ParentCpEvent, ParentCpState>(
            props: Self::Props,
        ) -> panoramix::elements::backend::ComponentHolder<Self, ParentCpEvent, ParentCpState>
        {
            panoramix::elements::backend::ComponentHolder::new(MyComponent, props)
        }

        fn name() -> &'static str {
            "MyComponent"
        }

        fn call_indirect<ParentCpEvent, ParentCpState>(
            &self,
            prev_state: (
                Self::LocalState,
                Option<panoramix::elements::any_element::AnyStateBox>,
            ),
            props: Self::Props,
        ) -> (
            panoramix::elements::component::ComponentOutput<
                Self::LocalEvent,
                Self::LocalState,
                panoramix::elements::any_element::VirtualDomBox<Self::LocalEvent, Self::LocalState>,
                ParentCpEvent,
                ParentCpState,
            >,
            (
                Self::LocalState,
                Option<panoramix::elements::any_element::AnyStateBox>,
            ),
        ) {
            panoramix::elements::backend::ComponentHolder::build_with(
                MyComponent,
                &MyComponent::render,
                prev_state,
                props,
            )
        }
    }

    use crate::element_tree::assign_empty_state_type;
    use crate::element_tree::Element;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn call_component() {
        let my_component = MyComponent::new(());
        assign_empty_state_type(&my_component);

        let (component_result, _state) = my_component.build(Default::default());
        assert_debug_snapshot!(component_result);

        //let prev_state = (999, Default::default());
        //let (component_result, component_state) = my_component.build(prev_state);
        //assert_eq!(component_state.0, 999);

        // TODO - local state
        // TODO - process_event
    }

    // TODO
    // - Widget test
    // - Events
}
