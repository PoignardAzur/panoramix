use crate::element_tree::{CompCtx, Element, NoEvent, NoState, VirtualDom};
use crate::glue::GlobalEventCx;

use crate::element_tree::ProcessEventCtx;
use crate::element_tree::ReconcileCtx;

use crate::elements::any_element::{AnyStateBox, ElementBox, VirtualDomBox};

use derivative::Derivative;
use std::fmt::Debug;

pub trait Component: Debug + Clone {
    type Props: Clone + Default + Debug + PartialEq + 'static;
    type LocalEvent: Clone + Debug + PartialEq + 'static;
    type LocalState: Clone + Default + Debug + PartialEq + 'static;

    fn new<ParentCpEvent: 'static, ParentCpState: 'static>(
        props: Self::Props,
    ) -> ElementBox<Self::LocalEvent, ParentCpEvent, ParentCpState>;

    fn name() -> &'static str;
}

#[derive(Derivative, Default, PartialEq, Eq, Hash)]
#[derivative(Clone(bound = "Comp::Props: Clone"))]
pub struct ComponentHolder<
    Comp: Component,
    ReturnedTree: Element<ParentCpEvent, ParentCpState, Event=Comp::LocalEvent>,
    CompFn: Clone + Fn(&CompCtx, Comp::Props) -> ReturnedTree,
    ParentCpEvent, ParentCpState
>
{
    component_fn: CompFn,
    props: Comp::Props,
    _marker: std::marker::PhantomData<(Comp, ParentCpEvent, ParentCpState)>,
}


#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Clone(bound = ""), Default(bound = "Child: Default"))]
pub struct ComponentOutputElem<
    ChildCpEvent: Clone + Debug + PartialEq,
    ChildCpState: Clone + Default + Debug + PartialEq,
    Child: Element<ChildCpEvent, ChildCpState>,
    ParentCpEvent,
    ParentCpState,
> {
    pub child: Child,
    pub name: &'static str,
    pub _markers: std::marker::PhantomData<(ParentCpEvent, ParentCpState, ChildCpEvent, ChildCpState)>,
}

#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Default(bound = "Child: Default"))]
pub struct ComponentOutput<
    ChildCpEvent: Clone + Debug + PartialEq,
    ChildCpState: Clone + Default + Debug + PartialEq,
    Child: VirtualDom<ChildCpEvent, ChildCpState>,
    ParentCpEvent,
    ParentCpState,
> {
    pub child: Child,
    pub name: &'static str,
    pub _markers: std::marker::PhantomData<(ParentCpEvent, ParentCpState, ChildCpEvent, ChildCpState)>,
}

// ---

impl<
    Comp: Component,
    ReturnedTree: Element<ParentCpEvent, ParentCpState, Event=Comp::LocalEvent>,
    CompFn: Clone + Fn(&CompCtx, Comp::Props) -> ReturnedTree,
    ParentCpEvent, ParentCpState
>
    ComponentHolder<Comp, ReturnedTree, CompFn, ParentCpEvent, ParentCpState>
{
    pub fn new(component_fn: CompFn, props: Comp::Props) -> Self {
        Self {
            component_fn,
            props,
            _marker: Default::default(),
        }
    }
}


impl<
    Comp: Component,
    ReturnedTree: Element<ParentCpEvent, ParentCpState, Event=Comp::LocalEvent>,
    CompFn: Clone + Fn(&CompCtx, Comp::Props) -> ReturnedTree,
    ParentCpEvent, ParentCpState
>
std::fmt::Debug for
    ComponentHolder<Comp, ReturnedTree, CompFn, ParentCpEvent, ParentCpState>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(Comp::name()).field(&self.props).finish()
    }
}

impl<
        ChildCpEvent: Clone + Debug + PartialEq,
        ChildCpState: Clone + Default + Debug + PartialEq,
        Child: Element<ChildCpEvent, ChildCpState>,
        ParentCpEvent,
        ParentCpState,
    > std::fmt::Debug
    for ComponentOutputElem<ChildCpEvent, ChildCpState, Child, ParentCpEvent, ParentCpState>
{
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(self.name)
            .field(&self.child)
        .finish()
    }
}

impl<
        ChildCpEvent: Clone + Debug + PartialEq,
        ChildCpState: Clone + Default + Debug + PartialEq,
        Child: VirtualDom<ChildCpEvent, ChildCpState>,
        ParentCpEvent,
        ParentCpState,
    > std::fmt::Debug
    for ComponentOutput<ChildCpEvent, ChildCpState, Child, ParentCpEvent, ParentCpState>
{
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(self.name)
            .field(&self.child)
        .finish()
    }
}

// ---

impl<
    Comp: Component,
    ReturnedTree: Element<ParentCpEvent, ParentCpState, Event=Comp::LocalEvent>,
    CompFn: Clone + Fn(&CompCtx, Comp::Props) -> ReturnedTree,
    ParentCpEvent, ParentCpState
>
Element<ParentCpEvent, ParentCpState> for
    ComponentHolder<Comp, ReturnedTree, CompFn, ParentCpEvent, ParentCpState>
{
    type Event = Comp::LocalEvent;
    type ComponentState = NoState;
    type AggregateChildrenState = ReturnedTree::AggregateChildrenState;
    type BuildOutput = ReturnedTree::BuildOutput;

    // TODO - add spans
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let default_state = Default::default();
        let local_state = ReturnedTree::get_component_state(&prev_state).unwrap_or(&default_state);

        let ctx = CompCtx {
            local_state: local_state,
        };
        let element_tree = (self.component_fn)(&ctx, self.props);

        element_tree.build(prev_state)
    }
}

/// ---

impl<
        ChildCpEvent: Clone + Debug + PartialEq,
        ChildCpState: Clone + Default + Debug + PartialEq + 'static,
        Child: Element<ChildCpEvent, ChildCpState>,
        ParentCpEvent,
        ParentCpState,
    > Element<ParentCpEvent, ParentCpState>
    for ComponentOutputElem<ChildCpEvent, ChildCpState, Child, ParentCpEvent, ParentCpState>
{
    type Event = ChildCpEvent;

    type ComponentState = ChildCpState;
    // TODO - Store Event queue somewhere else?
    type AggregateChildrenState = (
        Vec<ChildCpEvent>,
        ChildCpState,
        Child::AggregateChildrenState,
    );
    type BuildOutput = ComponentOutput<
        ChildCpEvent,
        ChildCpState,
        Child::BuildOutput,
        ParentCpEvent,
        ParentCpState,
    >;

    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (_, prev_local_state, children_prev_state) = prev_state;
        let (child, children_state) = self.child.build(children_prev_state);
        (
            ComponentOutput {
                child,
                name: self.name,
                _markers: Default::default(),
            },
            (vec![], prev_local_state, children_state),
        )
    }

    fn get_component_state(state: &Self::AggregateChildrenState) -> Option<&Self::ComponentState> {
        Some(&state.1)
    }
}

impl<
        ChildCpEvent: Clone + Debug + PartialEq,
        ChildCpState: Clone + Default + Debug + PartialEq,
        Child: VirtualDom<ChildCpEvent, ChildCpState>,
        ParentCpEvent,
        ParentCpState,
    > VirtualDom<ParentCpEvent, ParentCpState>
    for ComponentOutput<ChildCpEvent, ChildCpState, Child, ParentCpEvent, ParentCpState>
{
    type Event = ChildCpEvent;
    type AggregateChildrenState = (
        Vec<ChildCpEvent>,
        ChildCpState,
        Child::AggregateChildrenState,
    );
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
        children_state: &mut Self::AggregateChildrenState,
        _widget_seq: &mut Child::TargetWidgetSeq,
        _cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        let event_queue = &mut children_state.0;
        // TODO - this is a stack, not a queue; whatever, I'll use VecDeque later
        event_queue.pop()
    }

    fn process_event(
        &self,
        _comp_ctx: &mut ProcessEventCtx<ParentCpEvent, ParentCpState>,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        let mut ctx = ProcessEventCtx {
            event_queue: &mut children_state.0,
            state: &mut children_state.1,
        };
        self.child
            .process_event(&mut ctx, &mut children_state.2, widget_seq, cx)
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
        fn new<ParentCpEvent: 'static, ParentCpState: 'static>(
            props: MyPropsType,
        ) -> impl panoramix::Element<ParentCpEvent, ParentCpState, Event=panoramix::NoEvent> {
            <Self as panoramix::elements::component::Component>::new(props)
        }

        fn render<ParentCpEvent: 'static, ParentCpState: 'static>(
            _ctx: &panoramix::CompCtx,
            _my_props: MyPropsType,
        ) -> impl panoramix::Element<ParentCpEvent, ParentCpState, Event=panoramix::NoEvent> {
            let child = {
                panoramix::elements::EmptyElement::new()
            };
            panoramix::elements::component::ComponentOutputElem::<MyLocalEvent, MyLocalState, _, _, _> {
                child,
                name: "MyComponent",
                _markers: Default::default(),
            }
        }
    }

    impl panoramix::elements::component::Component for MyComponent {
        type Props = MyPropsType;
        type LocalState = MyLocalState;
        type LocalEvent = MyLocalEvent;

        fn new<ParentCpEvent: 'static, ParentCpState: 'static>(
            props: Self::Props,
        ) -> panoramix::elements::ElementBox<MyLocalEvent, ParentCpEvent, ParentCpState>
        {
            panoramix::elements::ElementBox::new(
                panoramix::elements::backend::ComponentHolder::<Self, _, _, ParentCpEvent, ParentCpState>::new(&MyComponent::render, props)
            )
        }

        fn name() -> &'static str {
            "MyComponent"
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
