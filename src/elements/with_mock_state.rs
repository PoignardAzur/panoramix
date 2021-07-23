use crate::ctx::{ProcessEventCtx, ReconcileCtx};
use crate::element_tree::{Element, VirtualDom};
use crate::glue::GlobalEventCx;
use crate::metadata::{NoEvent, NoState};

// Used for testing

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WithMockState<Child: Element>(pub Child);

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
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
    type ComponentState = NoState;
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
        prev_value: &Self,
        widget_seq: &mut Child::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.0.reconcile(&prev_value.0, widget_seq, ctx);
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
