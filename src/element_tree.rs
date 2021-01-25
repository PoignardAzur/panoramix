use crate::glue::GlobalEventCx;
use crate::widget_sequence::WidgetSequence;
use std::fmt::Debug;

pub trait ElementTree<ExplicitState>: Debug {
    type Event;
    type AggregateComponentState: Default + Debug;
    type BuildOutput: VirtualDom<
        ExplicitState,
        Event = Self::Event,
        AggregateComponentState = Self::AggregateComponentState,
    >;

    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState);
}

// TODO - Include documentation about what a Virtual DOM is and where the name comes from.
pub trait VirtualDom<ParentComponentState>: Debug {
    type Event;
    type AggregateComponentState: Default + Debug;

    // TODO - Might be superfluous
    type DomState;

    type TargetWidgetSeq: WidgetSequence;

    // update_value is intended to enable memoize-style HOC
    // where instead of returning a vdom node, it returns
    // something along the lines of struct KeepEverythingAsItWas()
    // Ugh. I'm not explaining this well.
    fn update_value(&mut self, other: Self);

    fn init_tree(&self) -> (Self::TargetWidgetSeq, Self::DomState);

    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Self::DomState,
        widget_seq: &mut Self::TargetWidgetSeq,
    ) -> Self::DomState;

    fn process_event(
        &self,
        explicit_state: &mut ParentComponentState,
        children_state: &mut Self::AggregateComponentState,
        dom_state: &mut Self::DomState,
        _cx: &mut GlobalEventCx,
    ) -> Option<Self::Event>;
}

// Useed in unit tests
#[allow(dead_code)]
pub(crate) fn assign_state_type<ExplicitState, Elem: ElementTree<ExplicitState>>(_elem: &Elem) {}

#[allow(dead_code)]
pub(crate) fn assign_empty_state_type(_elem: &impl ElementTree<()>) {}
