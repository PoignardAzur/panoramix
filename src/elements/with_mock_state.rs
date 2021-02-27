use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;

// Used for testing

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(
    Debug(bound = ""),
    Default(bound = "Child: Default"),
    Clone(bound = "Child: Clone")
)]
pub struct WithMockState<Child: Element<CpState, CpEvent>, CpState = (), CpEvent = NoEvent>(
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
pub struct WithMockStateData<Child: VirtualDom<CpState, CpEvent>, CpState = (), CpEvent = NoEvent>(
    pub Child,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MockState(String);

//
// --- IMPLS

impl<Child: Element<CpState, CpEvent>, CpState, CpEvent> WithMockState<Child, CpState, CpEvent> {
    pub fn new(child: Child) -> Self {
        WithMockState(child, Default::default(), Default::default())
    }
}

impl<Child: VirtualDom<CpState, CpEvent>, CpState, CpEvent>
    WithMockStateData<Child, CpState, CpEvent>
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

impl<CpState, CpEvent, Child: Element<CpState, CpEvent>> Element<CpState, CpEvent>
    for WithMockState<Child, CpState, CpEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = (MockState, Child::AggregateChildrenState);
    type BuildOutput = WithMockStateData<Child::BuildOutput, CpState, CpEvent>;

    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (mock_state, child_state) = prev_state;
        let (child, child_state) = self.0.build(child_state);
        (WithMockStateData::new(child), (mock_state, child_state))
    }
}

impl<CpState, CpEvent, Child: VirtualDom<CpState, CpEvent>> VirtualDom<CpState, CpEvent>
    for WithMockStateData<Child, CpState, CpEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = (MockState, Child::AggregateChildrenState);
    type TargetWidgetSeq = Child::TargetWidgetSeq;

    fn update_value(&mut self, other: Self) {
        self.0.update_value(other.0);
    }

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
        component_state: &mut CpState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Child::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<CpEvent> {
        self.0
            .process_event(component_state, &mut children_state.1, widget_seq, cx)
    }
}
