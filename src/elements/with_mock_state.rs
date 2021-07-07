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
pub struct WithMockState<Child: Element<CpEvent, CpState>, CpEvent = NoEvent, CpState = ()>(
    pub Child,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(
    Debug(bound = ""),
    Default(bound = "Child: Default"),
    Clone(bound = "Child: Clone")
)]
pub struct WithMockStateData<Child: VirtualDom<CpEvent, CpState>, CpEvent = NoEvent, CpState = ()>(
    pub Child,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MockState(String);

//
// --- IMPLS

impl<Child: Element<CpEvent, CpState>, CpEvent, CpState> WithMockState<Child, CpEvent, CpState> {
    pub fn new(child: Child) -> Self {
        WithMockState(child, Default::default(), Default::default())
    }
}

impl<Child: VirtualDom<CpEvent, CpState>, CpEvent, CpState>
    WithMockStateData<Child, CpEvent, CpState>
{
    pub fn new(child: Child) -> Self {
        WithMockStateData(child, Default::default(), Default::default())
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

impl<CpEvent, CpState, Child: Element<CpEvent, CpState>> Element<CpEvent, CpState>
    for WithMockState<Child, CpEvent, CpState>
{
    type Event = NoEvent;
    type ComponentState = crate::element_tree::NoState;
    type AggregateChildrenState = (MockState, Child::AggregateChildrenState);
    type BuildOutput = WithMockStateData<Child::BuildOutput, CpEvent, CpState>;

    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (mock_state, child_state) = prev_state;
        let (child, child_state) = self.0.build(child_state);
        (WithMockStateData::new(child), (mock_state, child_state))
    }
}

impl<CpEvent, CpState, Child: VirtualDom<CpEvent, CpState>> VirtualDom<CpEvent, CpState>
    for WithMockStateData<Child, CpEvent, CpState>
{
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
        comp_ctx: &mut ProcessEventCtx<CpEvent, CpState>,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Child::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        self.0
            .process_event(comp_ctx, &mut children_state.1, widget_seq, cx)
    }
}
