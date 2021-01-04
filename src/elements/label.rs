use crate::element_tree::{ElementTree, VirtualDom};
use crate::glue::{DruidAppData, GlobalEventCx, Id};
use crate::widgets::SingleWidget;

use druid::widget as druid_w;

pub struct Label<ExplicitState>(pub LabelTarget<ExplicitState>);

impl<ExplicitState> Label<ExplicitState> {
    pub fn new(text: impl Into<String>) -> Label<ExplicitState> {
        Label(LabelTarget(text.into(), Default::default()))
    }
}

#[derive(Debug, PartialEq)]
pub struct LabelTarget<ParentComponentState>(
    pub String,
    pub std::marker::PhantomData<ParentComponentState>,
);

//
// --- TRAIT IMPLS

impl<ExplicitState> ElementTree<ExplicitState> for Label<ExplicitState> {
    type Event = ();
    type AggregateComponentState = ();
    type BuildOutput = LabelTarget<ExplicitState>;

    fn build(self, _prev_state: ()) -> (LabelTarget<ExplicitState>, ()) {
        (self.0, ())
    }
}

impl<ParentComponentState> VirtualDom<ParentComponentState> for LabelTarget<ParentComponentState> {
    type Event = ();
    type DomState = Id;
    type AggregateComponentState = ();

    type TargetWidgetSeq = SingleWidget<druid_w::Label<DruidAppData>>;

    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    fn init_tree(&self) -> (Self::TargetWidgetSeq, Id) {
        let text = &self.0;
        let id = Id::new();
        let label = druid_w::Label::new(text.clone());
        (SingleWidget::new(label), id)
    }

    fn apply_diff(&self, _other: &Self, prev_state: Id, widget: &mut Self::TargetWidgetSeq) -> Id {
        let text = &self.0;
        widget.0.widget_mut().set_text(text.clone());
        prev_state
    }

    fn process_event(
        &self,
        _explicit_state: &mut ParentComponentState,
        _children_state: &mut (),
        _dom_state: &mut Id,
        _cx: &mut GlobalEventCx,
    ) -> Option<()> {
        None
    }
}
