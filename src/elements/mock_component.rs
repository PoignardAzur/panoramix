use crate::ctx::{ProcessEventCtx, ReconcileCtx};
use crate::element_tree::{Element, VirtualDom};
use crate::glue::{Action, GlobalEventCx, WidgetId};
use crate::metadata::{NoEvent, NoState};
use crate::widgets::ButtonWidget;

use tracing::trace;

/// Mocks a component with a single button, and some local state.
///
/// Used to test element identity, and local state stability.
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MockComponent {
    reserved_widget_id: Option<WidgetId>,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MockComponentData {
    reserved_widget_id: Option<WidgetId>,
    clicks: u32,
}

/// Local state of [MockComponent]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MockState {
    value: String,
    clicks: u32,
}

//
// --- IMPLS

impl MockComponent {
    pub fn new() -> Self {
        MockComponent {
            reserved_widget_id: None,
        }
    }

    /// The button created by this element always has the same id. If two widgets are created
    /// with the same id (for instance, because the same button is returned twice because of
    /// a copy-paste error), impredictable behavior may ensue.
    pub fn with_reserved_id(self, widget_id: WidgetId) -> Self {
        MockComponent {
            reserved_widget_id: Some(widget_id),
            ..self
        }
    }
}

impl MockComponentData {
    pub fn new(clicks: u32) -> Self {
        MockComponentData {
            reserved_widget_id: None,
            clicks,
        }
    }
}

impl MockState {
    pub fn new(text: impl Into<String>) -> MockState {
        MockState {
            value: text.into(),
            clicks: 0,
        }
    }
}

impl Default for MockState {
    fn default() -> Self {
        MockState {
            value: String::from("default-value"),
            clicks: 0,
        }
    }
}

impl Element for MockComponent {
    type Event = NoEvent;
    type ComponentState = NoState;
    type AggregateChildrenState = MockState;
    type BuildOutput = MockComponentData;

    fn build(self, prev_state: MockState) -> (Self::BuildOutput, MockState) {
        (
            MockComponentData {
                clicks: prev_state.clicks,
                reserved_widget_id: self.reserved_widget_id,
            },
            prev_state,
        )
    }
}

impl VirtualDom for MockComponentData {
    type Event = NoEvent;
    type AggregateChildrenState = MockState;
    type TargetWidgetSeq = ButtonWidget;

    fn init_tree(&self) -> ButtonWidget {
        let id = self.reserved_widget_id.unwrap_or_else(WidgetId::next);
        ButtonWidget::new(String::from("clickme"), Default::default(), id)
    }

    fn reconcile(&self, _prev_value: &Self, _widget: &mut ButtonWidget, _ctx: &mut ReconcileCtx) {}

    fn process_event(
        &self,
        _comp_ctx: &mut ProcessEventCtx,
        children_state: &mut MockState,
        widget: &mut ButtonWidget,
        cx: &mut GlobalEventCx,
    ) {
        let id = widget.id();
        if let Some(Action::Clicked) = cx.app_data.dequeue_action(id) {
            trace!("Processed MockComponent button press");
            children_state.clicks += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_harness::Harness;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn new_mock_component() {
        let component = MockComponent::new();
        let (component_data, mock_state) = component.clone().build(MockState::new("FOOBAR"));

        assert_debug_snapshot!(component);
        assert_debug_snapshot!(component_data);
        assert_debug_snapshot!(mock_state);
    }

    #[test]
    fn mock_component_button_widget() {
        let button = MockComponent::new();

        Harness::run_test_window(button, |harness| {
            let root_state = harness.get_root_debug_state();
            assert_debug_snapshot!(root_state);
        });
    }
}
