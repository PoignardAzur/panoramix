use crate::element_tree::{ElementTree, VirtualDom};
use crate::glue::{DruidAppData, GlobalEventCx, Id};
use crate::widgets::SingleWidget;

use derivative::Derivative;
use druid::widget as druid_w;
use tracing::instrument;

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""))]
pub struct Label<ExplicitState>(pub String, pub std::marker::PhantomData<ExplicitState>);

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""))]
pub struct LabelData<ParentComponentState>(
    pub String,
    pub std::marker::PhantomData<ParentComponentState>,
);

//
// --- IMPLS

impl<ExplicitState> Label<ExplicitState> {
    pub fn new(text: impl Into<String>) -> Label<ExplicitState> {
        Label(text.into(), Default::default())
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

    #[instrument(name = "Label", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (LabelData<ExplicitState>, ()) {
        (LabelData(self.0, Default::default()), ())
    }
}

impl<ParentComponentState> VirtualDom<ParentComponentState> for LabelData<ParentComponentState> {
    type Event = ();
    type DomState = Id;
    type AggregateComponentState = ();

    type TargetWidgetSeq = SingleWidget<druid_w::Label<DruidAppData>>;

    #[instrument(name = "Label", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Label", skip(self))]
    fn init_tree(&self) -> (Self::TargetWidgetSeq, Id) {
        let text = &self.0;
        let id = Id::new();
        let label = druid_w::Label::new(text.clone());
        (SingleWidget::new(label), id)
    }

    #[instrument(name = "Label", skip(self, other, prev_state, widget))]
    fn apply_diff(&self, other: &Self, prev_state: Id, widget: &mut Self::TargetWidgetSeq) -> Id {
        let text = &self.0;
        let prev_text = &other.0;
        if text != prev_text {
            widget.0.widget_mut().set_text(text.clone());
        }
        prev_state
    }

    #[instrument(
        name = "Label",
        skip(self, _explicit_state, _children_state, _dom_state, _cx)
    )]
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
    use crate::element_tree::assign_empty_state_type;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn new_label() {
        let label = Label::new("Hello");
        let (label_data, ()) = label.clone().build(());

        assert_debug_snapshot!(label);
        assert_debug_snapshot!(label_data);

        assert_eq!(label_data, LabelData::new("Hello"));

        assign_empty_state_type(&label);
    }

    // TODO
    // - Widget test
}
