use crate::element_tree::{ElementTree, VirtualDom};
use crate::glue::GlobalEventCx;

// Used for testing

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct WithMockState<Child: ElementTree<ExplicitState>, ExplicitState>(
    pub Child,
    pub std::marker::PhantomData<ExplicitState>,
);

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct WithMockStateData<Child: VirtualDom<ParentComponentState>, ParentComponentState>(
    pub Child,
    pub std::marker::PhantomData<ParentComponentState>,
);

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct MockState(String);

//
// --- IMPLS

impl<Child: ElementTree<ExplicitState>, ExplicitState> WithMockState<Child, ExplicitState> {
    pub fn new(child: Child) -> Self {
        WithMockState(child, Default::default())
    }
}

impl<Child: VirtualDom<ParentComponentState>, ParentComponentState>
    WithMockStateData<Child, ParentComponentState>
{
    pub fn new(child: Child) -> Self {
        WithMockStateData(child, Default::default())
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

impl<Child: ElementTree<ExplicitState>, ExplicitState> ElementTree<ExplicitState>
    for WithMockState<Child, ExplicitState>
{
    type Event = Child::Event;
    type AggregateComponentState = (MockState, Child::AggregateComponentState);
    type BuildOutput = WithMockStateData<Child::BuildOutput, ExplicitState>;

    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState) {
        let (mock_state, child_state) = prev_state;
        let (child, child_state) = self.0.build(child_state);
        (WithMockStateData::new(child), (mock_state, child_state))
    }
}

impl<Child: VirtualDom<ParentComponentState>, ParentComponentState> VirtualDom<ParentComponentState>
    for WithMockStateData<Child, ParentComponentState>
{
    type Event = Child::Event;
    type DomState = Child::DomState;
    type AggregateComponentState = (MockState, Child::AggregateComponentState);

    type TargetWidgetSeq = Child::TargetWidgetSeq;

    fn update_value(&mut self, other: Self) {
        self.0.update_value(other.0);
    }

    fn init_tree(&self) -> (Child::TargetWidgetSeq, Child::DomState) {
        self.0.init_tree()
    }

    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Child::DomState,
        widget: &mut Self::TargetWidgetSeq,
    ) -> Child::DomState {
        self.0.apply_diff(&other.0, prev_state, widget)
    }

    fn process_event(
        &self,
        explicit_state: &mut ParentComponentState,
        children_state: &mut Self::AggregateComponentState,
        dom_state: &mut Child::DomState,
        cx: &mut GlobalEventCx,
    ) -> Option<Child::Event> {
        self.0
            .process_event(explicit_state, &mut children_state.1, dom_state, cx)
    }
}
