use crate::ctx::ReconcileCtx;
use crate::element_tree::{Element, VirtualDom};
use crate::flex::FlexParams;
use crate::glue::DruidAppData;
use crate::metadata::{NoEvent, NoState};
use crate::widgets::SingleWidget;

use druid::widget as druid_w;

use tracing::instrument;

/// A text label.
///
/// ## Events
///
/// Doesn't emit events.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Label {
    pub text: String,
    pub flex: FlexParams,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LabelData {
    pub text: String,
    pub flex: FlexParams,
}

//
// --- IMPLS

impl Label {
    /// Build a text label.
    pub fn new(text: impl Into<String>) -> Label {
        Label {
            text: text.into(),
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
        }
    }

    /// Change the way the label's size is calculated
    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        Label {
            flex: flex_params,
            ..self
        }
    }

    // TODO
    /// Used for unit tests.
    pub fn with_mock_state(self) -> super::internals::WithMockState<Self> {
        super::internals::WithMockState::new(self)
    }
}

impl LabelData {
    pub fn new(text: impl Into<String>) -> LabelData {
        LabelData {
            text: text.into(),
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
        }
    }

    pub fn with_mock_state(self) -> super::internals::WithMockStateData<Self> {
        super::internals::WithMockStateData::new(self)
    }
}

impl Element for Label {
    type Event = NoEvent;
    type ComponentState = NoState;
    type AggregateChildrenState = ();
    type BuildOutput = LabelData;

    #[instrument(name = "Label", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (LabelData, ()) {
        (
            LabelData {
                text: self.text,
                flex: self.flex,
            },
            (),
        )
    }
}

impl VirtualDom for LabelData {
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type TargetWidgetSeq = SingleWidget<druid_w::Label<DruidAppData>>;

    #[instrument(name = "Label", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        let label = druid_w::Label::new(self.text.clone());
        SingleWidget::new(label, self.flex)
    }

    #[instrument(name = "Label", skip(self, other, widget, ctx))]
    fn reconcile(&self, other: &Self, widget: &mut Self::TargetWidgetSeq, ctx: &mut ReconcileCtx) {
        if self.text != other.text {
            widget.pod.widget_mut().set_text(self.text.clone());
            widget.request_druid_update(ctx.event_ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn new_label() {
        let label = Label::new("Hello");
        let (label_data, ()) = label.clone().build(());

        assert_debug_snapshot!(label);
        assert_debug_snapshot!(label_data);

        assert_eq!(label_data, LabelData::new("Hello"));
    }

    #[test]
    fn test_label_widget() {
        use crate::test_harness::Harness;

        let label = Label::new("Hello");

        Harness::run_test_window(label, |harness| {
            let label_state = harness.get_root_debug_state();
            assert_debug_snapshot!(label_state);

            let new_label = Label::new("World");
            harness.update_root_element(new_label);

            let label_state_2 = harness.get_root_debug_state();
            assert_debug_snapshot!(label_state_2);
        });
    }
}
