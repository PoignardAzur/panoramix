use crate::glue::{DruidAppData, GlobalEventCx};
use crate::widgets::SingleWidget;

use crate::element_tree::{ElementTree, VirtualDom};
use druid::widget as druid_w;
use druid::WidgetPod;

pub struct EmptyElement<ExplicitState = ()>(pub std::marker::PhantomData<ExplicitState>);

impl<ExplicitState> EmptyElement<ExplicitState> {
    pub fn new() -> EmptyElement<ExplicitState> {
        EmptyElement(Default::default())
    }
}

// Instead of doing multiple implementations of TupleComponent for different tuple sizes,
// I'm being lazy and doing one implem for a huge tuple, and stuffing it with EmptyElement
// when using it. It's *a lot* easier.
pub struct EmptyElementTarget<ParentComponentState>(
    pub std::marker::PhantomData<ParentComponentState>,
);

impl<ExplicitState> ElementTree<ExplicitState> for EmptyElement<ExplicitState> {
    type Event = ();
    type AggregateComponentState = ();
    type BuildOutput = EmptyElementTarget<ExplicitState>;

    fn build(self, _prev_state: ()) -> (EmptyElementTarget<ExplicitState>, ()) {
        (EmptyElementTarget(Default::default()), ())
    }
}

impl<ParentComponentState> VirtualDom<ParentComponentState>
    for EmptyElementTarget<ParentComponentState>
{
    type Event = ();
    type DomState = ();
    type AggregateComponentState = ();

    type TargetWidgetSeq = SingleWidget<druid_w::Flex<DruidAppData>>;

    fn update_value(&mut self, _other: Self) {}

    fn init_tree(&self) -> (Self::TargetWidgetSeq, ()) {
        (SingleWidget(WidgetPod::new(druid_w::Flex::row())), ())
    }

    fn apply_diff(
        &self,
        _other: &Self,
        _prev_state: (),
        _widget: &mut Self::TargetWidgetSeq,
    ) -> () {
    }

    fn process_event(
        &self,
        _explicit_state: &mut ParentComponentState,
        _children_state: &mut (),
        _dom_state: &mut (),
        _cx: &mut GlobalEventCx,
    ) -> Option<()> {
        return None;
    }
}
