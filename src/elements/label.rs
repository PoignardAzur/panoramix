use crate::element_tree::{ElementTree, VirtualDom};
use crate::glue::{DruidAppData, GlobalEventCx, Id};
use crate::widgets::SingleWidget;

use druid::widget as druid_w;

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Label<ExplicitState>(pub LabelData<ExplicitState>);

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct LabelData<ParentComponentState>(
    pub String,
    pub std::marker::PhantomData<ParentComponentState>,
);

//
// --- IMPLS

impl<ExplicitState> Label<ExplicitState> {
    pub fn new(text: impl Into<String>) -> Label<ExplicitState> {
        Label(LabelData(text.into(), Default::default()))
    }

    pub fn with_mock_state(self) -> super::WithMockState<Self, ExplicitState> {
        super::WithMockState::new(self)
    }
}

impl<ParentComponentState> LabelData<ParentComponentState> {
    pub fn new(text: impl Into<String>) -> LabelData<ParentComponentState> {
        LabelData(text.into(), Default::default())
    }

    pub fn with_mock_state(self) -> super::WithMockStateData<Self, ParentComponentState> {
        super::WithMockStateData::new(self)
    }
}

impl<ExplicitState> ElementTree<ExplicitState> for Label<ExplicitState> {
    type Event = ();
    type AggregateComponentState = ();
    type BuildOutput = LabelData<ExplicitState>;

    fn build(self, _prev_state: ()) -> (LabelData<ExplicitState>, ()) {
        (self.0, ())
    }
}

impl<ParentComponentState> VirtualDom<ParentComponentState> for LabelData<ParentComponentState> {
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

    fn apply_diff(&self, other: &Self, prev_state: Id, widget: &mut Self::TargetWidgetSeq) -> Id {
        let text = &self.0;
        let prev_text = &other.0;
        if text != prev_text {
            widget.0.widget_mut().set_text(text.clone());
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_label() {
        let label = Label::<()>::new("Hello");
        let (label_data, ()) = label.clone().build(());
        assert_eq!(
            label,
            Label(LabelData(String::from("Hello"), Default::default()))
        );
        assert_eq!(
            label_data,
            LabelData(String::from("Hello"), Default::default())
        );
    }

    #[test]
    fn new_label_data() {
        let label_data = LabelData::<()>::new("Hello");
        assert_eq!(
            label_data,
            LabelData(String::from("Hello"), Default::default())
        );
    }

    // TODO
    // - Widget test
}
