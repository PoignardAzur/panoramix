use crate::element_tree::{ElementTree, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;

use derivative::Derivative;
use std::fmt::Debug;
use tracing::instrument;

pub struct ComponentCaller<
    ChildComponentState: Default + Debug,
    ChildComponentEvent,
    Props,
    ReturnedTree: ElementTree<ChildComponentState, ChildComponentEvent>,
    Comp: Fn(&ChildComponentState, Props) -> ReturnedTree,
    ParentComponentState = (),
    ParentComponentEvent = NoEvent,
> {
    pub component: Comp,
    pub props: Props,
    pub _parent_state: std::marker::PhantomData<ParentComponentState>,
    pub _parent_event: std::marker::PhantomData<ParentComponentEvent>,
    pub _child_state: std::marker::PhantomData<ChildComponentState>,
    pub _child_event: std::marker::PhantomData<ChildComponentEvent>,
    pub _returned_tree: std::marker::PhantomData<ReturnedTree>,
}

#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = "Child: Default"))]
pub struct ComponentCallerData<
    ChildComponentState: Default + Debug,
    ChildComponentEvent,
    Child: VirtualDom<ChildComponentState, ChildComponentEvent>,
    ParentComponentState,
    ParentComponentEvent,
>(
    Child,
    std::marker::PhantomData<ParentComponentState>,
    std::marker::PhantomData<ParentComponentEvent>,
    std::marker::PhantomData<ChildComponentState>,
    std::marker::PhantomData<ChildComponentEvent>,
);

impl<
        ParentComponentState,
        ParentComponentEvent,
        ChildComponentState: Default + Debug,
        ChildComponentEvent,
        Props,
        ReturnedTree: ElementTree<ChildComponentState, ChildComponentEvent>,
        Comp: Fn(&ChildComponentState, Props) -> ReturnedTree,
    >
    ComponentCaller<
        ChildComponentState,
        ChildComponentEvent,
        Props,
        ReturnedTree,
        Comp,
        ParentComponentState,
        ParentComponentEvent,
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
        ParentComponentState,
        ParentComponentEvent,
        ChildComponentState: Default + Debug,
        ChildComponentEvent,
        Props,
        ReturnedTree: ElementTree<ChildComponentState, ChildComponentEvent>,
        Comp: Fn(&ChildComponentState, Props) -> ReturnedTree,
    > std::fmt::Debug
    for ComponentCaller<
        ChildComponentState,
        ChildComponentEvent,
        Props,
        ReturnedTree,
        Comp,
        ParentComponentState,
        ParentComponentEvent,
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
        ParentComponentState,
        ParentComponentEvent,
        // TODO - remove?
        ChildComponentState: Default + Debug,
        ChildComponentEvent,
        Props,
        ReturnedTree: ElementTree<ChildComponentState, ChildComponentEvent>,
        Comp: Fn(&ChildComponentState, Props) -> ReturnedTree,
    > ElementTree<ParentComponentState, ParentComponentEvent>
    for ComponentCaller<
        ChildComponentState,
        ChildComponentEvent,
        Props,
        ReturnedTree,
        Comp,
        ParentComponentState,
        ParentComponentEvent,
    >
{
    type Event = ReturnedTree::Event;
    type AggregateChildrenState = (ChildComponentState, ReturnedTree::AggregateChildrenState);
    type BuildOutput = ComponentCallerData<
        ChildComponentState,
        ChildComponentEvent,
        ReturnedTree::BuildOutput,
        ParentComponentState,
        ParentComponentEvent,
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
        ParentComponentState,
        ParentComponentEvent,
        ChildComponentState: Default + Debug,
        ChildComponentEvent,
        Child: VirtualDom<ChildComponentState, ChildComponentEvent>,
    > VirtualDom<ParentComponentState, ParentComponentEvent>
    for ComponentCallerData<
        ChildComponentState,
        ChildComponentEvent,
        Child,
        ParentComponentState,
        ParentComponentEvent,
    >
{
    type Event = Child::Event;
    type AggregateChildrenState = (ChildComponentState, Child::AggregateChildrenState);
    type TargetWidgetSeq = Child::TargetWidgetSeq;

    #[instrument(name = "Component", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        self.0.update_value(other.0);
    }

    #[instrument(name = "Component", skip(self))]
    fn init_tree(&self) -> Child::TargetWidgetSeq {
        self.0.init_tree()
    }

    #[instrument(name = "Component", skip(self, other, widget_seq))]
    fn reconcile(&self, other: &Self, widget_seq: &mut Child::TargetWidgetSeq) {
        self.0.reconcile(&other.0, widget_seq);
    }

    #[instrument(
        name = "Component",
        skip(self, _component_state, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        _component_state: &mut ParentComponentState,
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
    use crate::element_tree::NoEvent;
    use crate::element_tree_ext::ElementTreeExt;
    use crate::elements::{Button, ButtonPressed, EventEnum, Label};
    use crate::make_row;

    use insta::assert_debug_snapshot;
    use test_env_log::test;

    type MyEvent = EventEnum<ButtonPressed, NoEvent>;

    // TODO - add tracing, and detect when this function is called by tests
    fn my_component(state: &u16, props: i64) -> impl ElementTree<u16, Event = MyEvent> {
        make_row!(
            Button::new("Press me").with_event(|state: &mut u16, _| {
                *state += 1;
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
