use crate::glue::GlobalEventCx;
use crate::widget_sequence::WidgetSequence;

use std::fmt::Debug;

// TODO - must-use
// TODO - Default + Debug bounds
pub trait ElementTree<ComponentState = (), ComponentEvent = NoEvent>: Debug {
    type Event;

    type AggregateChildrenState: Default + Debug;
    type BuildOutput: VirtualDom<
        ComponentState,
        ComponentEvent,
        Event = Self::Event,
        AggregateChildrenState = Self::AggregateChildrenState,
    >;

    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState);
}

// TODO - Include documentation about what a Virtual DOM is and where the name comes from.
pub trait VirtualDom<ComponentState, ComponentEvent>: Debug {
    type AggregateChildrenState: Default + Debug;
    type TargetWidgetSeq: WidgetSequence;

    type Event;

    // update_value is intended to enable memoize-style HOC
    // where instead of returning a vdom node, it returns
    // something along the lines of struct KeepEverythingAsItWas()
    // Ugh. I'm not explaining this well.
    fn update_value(&mut self, other: Self);

    fn init_tree(&self) -> Self::TargetWidgetSeq;

    fn reconcile(&self, other: &Self, widget_seq: &mut Self::TargetWidgetSeq);

    fn process_event(
        &self,
        component_state: &mut ComponentState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoEvent {}

// Used in unit tests
#[allow(dead_code)]
pub(crate) fn assign_empty_state_type(_elem: &impl ElementTree<(), NoEvent>) {}

#[allow(dead_code)]
pub(crate) fn assign_state_type<
    ComponentState,
    ComponentEvent,
    Elem: ElementTree<ComponentState, ComponentEvent>,
>(
    _elem: &Elem,
) {
}
