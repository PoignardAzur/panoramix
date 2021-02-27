use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::glue::DruidAppData;
use crate::widgets::flex::FlexParams;
use crate::widgets::SingleWidget;

use crate::element_tree::ReconcileCtx;
use druid::widget as druid_w;

use derivative::Derivative;
use tracing::instrument;

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct Label<CpState = (), CpEvent = NoEvent>(
    pub String,
    pub FlexParams,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct LabelData<CpState = (), CpEvent = NoEvent>(
    pub String,
    pub FlexParams,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

//
// --- IMPLS

impl<CpState, CpEvent> Label<CpState, CpEvent> {
    pub fn new(text: impl Into<String>) -> Label<CpState, CpEvent> {
        Label(
            text.into(),
            FlexParams {
                flex: 1.0,
                alignment: None,
            },
            Default::default(),
            Default::default(),
        )
    }

    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        Label(self.0, flex_params, Default::default(), Default::default())
    }

    pub fn with_mock_state(self) -> super::WithMockState<Self, CpState, CpEvent> {
        super::WithMockState::new(self)
    }
}

impl<CpState, CpEvent> LabelData<CpState, CpEvent> {
    pub fn new(text: impl Into<String>) -> LabelData<CpState, CpEvent> {
        LabelData(
            text.into(),
            FlexParams {
                flex: 1.0,
                alignment: None,
            },
            Default::default(),
            Default::default(),
        )
    }

    pub fn with_mock_state(self) -> super::WithMockStateData<Self, CpState, CpEvent> {
        super::WithMockStateData::new(self)
    }
}

impl<CpState, CpEvent> Element<CpState, CpEvent> for Label<CpState, CpEvent> {
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type BuildOutput = LabelData<CpState, CpEvent>;

    #[instrument(name = "Label", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (LabelData<CpState, CpEvent>, ()) {
        (
            LabelData(self.0, self.1, Default::default(), Default::default()),
            (),
        )
    }
}

impl<CpState, CpEvent> VirtualDom<CpState, CpEvent> for LabelData<CpState, CpEvent> {
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type TargetWidgetSeq = SingleWidget<druid_w::Label<DruidAppData>>;

    #[instrument(name = "Label", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Label", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        let text = &self.0;
        let label = druid_w::Label::new(text.clone());
        SingleWidget::new(label, self.1)
    }

    #[instrument(name = "Label", skip(self, other, widget, ctx))]
    fn reconcile(&self, other: &Self, widget: &mut Self::TargetWidgetSeq, ctx: &mut ReconcileCtx) {
        let text = &self.0;
        let prev_text = &other.0;
        if text != prev_text {
            widget.pod.widget_mut().set_text(text.clone());
            widget.request_druid_update(ctx);
        }
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
