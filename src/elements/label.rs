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
pub struct Label<CpState = (), CpEvent = NoEvent> {
    pub text: String,
    pub flex: FlexParams,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpState, CpEvent)>,
}

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct LabelData<CpState = (), CpEvent = NoEvent> {
    pub text: String,
    pub flex: FlexParams,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpState, CpEvent)>,
}

//
// --- IMPLS

impl<CpState, CpEvent> Label<CpState, CpEvent> {
    pub fn new(text: impl Into<String>) -> Label<CpState, CpEvent> {
        Label {
            text: text.into(),
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
            _markers: Default::default(),
        }
    }

    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        Label {
            flex: flex_params,
            ..self
        }
    }

    pub fn with_mock_state(self) -> super::WithMockState<Self, CpState, CpEvent> {
        super::WithMockState::new(self)
    }
}

impl<CpState, CpEvent> LabelData<CpState, CpEvent> {
    pub fn new(text: impl Into<String>) -> LabelData<CpState, CpEvent> {
        LabelData {
            text: text.into(),
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
            _markers: Default::default(),
        }
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
            LabelData {
                text: self.text,
                flex: self.flex,
                _markers: Default::default(),
            },
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
        let label = druid_w::Label::new(self.text.clone());
        SingleWidget::new(label, self.flex)
    }

    #[instrument(name = "Label", skip(self, other, widget, ctx))]
    fn reconcile(&self, other: &Self, widget: &mut Self::TargetWidgetSeq, ctx: &mut ReconcileCtx) {
        if self.text != other.text {
            widget.pod.widget_mut().set_text(self.text.clone());
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
