use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;

use crate::element_tree::ProcessEventCtx;
use crate::element_tree::ReconcileCtx;

use derivative::Derivative;

// Used for testing

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(
    Debug(bound = ""),
    Default(bound = "Child: Default"),
    Clone(bound = "Child: Clone")
)]
pub struct WithMockState<Child: Element>(pub Child);

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(
    Debug(bound = ""),
    Default(bound = "Child: Default"),
    Clone(bound = "Child: Clone")
)]
pub struct WithMockStateData<Child: VirtualDom>(pub Child);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MockState(String);

//
// --- IMPLS

impl<Child: Element> WithMockState<Child> {
    pub fn new(child: Child) -> Self {
        WithMockState(child)
    }
}

impl<Child: VirtualDom> WithMockStateData<Child> {
    pub fn new(child: Child) -> Self {
        WithMockStateData(child)
    }
}

impl MockState {
    pub fn new(text: impl Into<String>) -> MockState {
        MockState(text.into())
    }
}

impl Default for MockState {
    fn default() -> Self {
        MockState(String::from("HelloWorld"))
    }
}

impl<Child: Element> Element for WithMockState<Child> {
    type Event = NoEvent;
    type ComponentState = crate::element_tree::NoState;
    type AggregateChildrenState = (MockState, Child::AggregateChildrenState);
    type BuildOutput = WithMockStateData<Child::BuildOutput>;

    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (mock_state, child_state) = prev_state;
        let (child, child_state) = self.0.build(child_state);
        (WithMockStateData::new(child), (mock_state, child_state))
    }
}

impl<Child: VirtualDom> VirtualDom for WithMockStateData<Child> {
    type Event = NoEvent;
    type AggregateChildrenState = (MockState, Child::AggregateChildrenState);
    type TargetWidgetSeq = Child::TargetWidgetSeq;

    fn init_tree(&self) -> Child::TargetWidgetSeq {
        self.0.init_tree()
    }

    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Child::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.0.reconcile(&other.0, widget_seq, ctx);
    }

    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Child::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        self.0
            .process_event(comp_ctx, &mut children_state.1, widget_seq, cx)
    }
}
