use crate::ctx::ReconcileCtx;
use crate::element_tree::{Element, ElementExt, VirtualDom};
use crate::flex::FlexParams;
use crate::glue::{Action, GlobalEventCx, WidgetId};
use crate::metadata::{Metadata, NoState};
use crate::widgets::{CheckboxWidget, SingleCheckboxWidget};

use tracing::{instrument, trace};

// TODO - Handle the anti-pattern where the user does something like
// Checkbox::new(false)
// In other words, enforce two-ways bindings

/// A checkbox with a text label.
///
/// ## Events
///
/// Emits [Toggled] events.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Checkbox {
    pub text: String,
    pub value: bool,
    pub flex: FlexParams,
    pub reserved_widget_id: Option<WidgetId>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CheckboxData {
    pub text: String,
    pub value: bool,
    pub flex: FlexParams,
    pub reserved_widget_id: Option<WidgetId>,
}

/// Event emitted when a [Checkbox] is clicked.
///
/// Note: Might hold data like "mouse position" or "checkbox id" future versions.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Toggled {
    pub new_value: bool,
}

//
// --- IMPLS

impl Checkbox {
    /// Build a checkbox with the given label.
    ///
    /// Use the [.on_toggled](Checkbox::on_toggled) method to provide a closure to be called when the box is toggled.
    pub fn new(text: impl Into<String>, value: bool) -> Self {
        Checkbox {
            text: text.into(),
            value,
            flex: FlexParams {
                flex: None,
                alignment: None,
            },
            reserved_widget_id: None,
        }
    }

    /// Change the way the checkbox's size is calculated
    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        Checkbox {
            flex: flex_params,
            ..self
        }
    }

    /// For unit tests only.
    ///
    /// The checkbox created by this element always has the same id. If two widgets are created
    /// with the same id (for instance, because the same checkbox is returned twice because of
    /// a copy-paste error), impredictable behavior may ensue.
    pub fn with_reserved_id(self, widget_id: WidgetId) -> Self {
        Checkbox {
            reserved_widget_id: Some(widget_id),
            ..self
        }
    }

    /// Provide a closure to be called when this checkbox is toggled.
    pub fn on_toggled<ComponentEvent: 'static, ComponentState: 'static>(
        self,
        md: Metadata<ComponentEvent, ComponentState>,
        callback: impl Fn(&mut ComponentState, Toggled) + Clone + 'static,
    ) -> impl Element {
        self.on(md, callback)
    }
}

impl Element for Checkbox {
    type Event = Toggled;

    type ComponentState = NoState;
    type AggregateChildrenState = ();
    type BuildOutput = CheckboxData;

    #[instrument(name = "Checkbox", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (CheckboxData, ()) {
        (
            CheckboxData {
                text: self.text,
                value: self.value,
                flex: self.flex,
                reserved_widget_id: self.reserved_widget_id,
            },
            (),
        )
    }
}

impl VirtualDom for CheckboxData {
    type Event = Toggled;
    type AggregateChildrenState = ();

    type TargetWidgetSeq = SingleCheckboxWidget;

    #[instrument(name = "Checkbox", skip(self))]
    fn init_tree(&self) -> SingleCheckboxWidget {
        let id = self.reserved_widget_id.unwrap_or_else(WidgetId::next);
        SingleCheckboxWidget::new(
            CheckboxWidget::new(self.text.clone(), self.value, id),
            self.flex,
        )
    }

    #[instrument(name = "Checkbox", skip(self, prev_value, widget, ctx))]
    fn reconcile(
        &self,
        prev_value: &Self,
        widget: &mut SingleCheckboxWidget,
        ctx: &mut ReconcileCtx,
    ) {
        let checkbox_widget = widget.widget_mut();
        if self.text != prev_value.text {
            // TODO
            //checkbox_widget.pod.widget_mut().set_text(self.text.clone());
        }
        checkbox_widget.value = self.value;
        // TODO - check diff with previous value
        widget.request_druid_update(ctx.event_ctx);
        widget.widget_mut().request_druid_update(ctx.event_ctx);
    }

    #[instrument(name = "Checkbox", skip(self, _children_state, widget, cx))]
    fn process_local_event(
        &self,
        _children_state: &mut Self::AggregateChildrenState,
        widget: &mut SingleCheckboxWidget,
        cx: &mut GlobalEventCx,
    ) -> Option<Toggled> {
        // FIXME - Rework event dispatching
        let id = widget.widget().id();
        if let Some(Action::Clicked) = cx.app_data.dequeue_action(id) {
            let new_value = widget.widget().value;
            trace!("Processed checkbox toggle: {}", new_value);
            Some(Toggled { new_value })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_harness::Harness;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    // TODO - Test flex params?

    #[test]
    fn new_checkbox() {
        let checkbox = Checkbox::new("Hello", false);
        let (checkbox_data, ()) = checkbox.clone().build(());

        assert_debug_snapshot!(checkbox);
        assert_debug_snapshot!(checkbox_data);

        assert_eq!(
            checkbox_data,
            CheckboxData {
                text: String::from("Hello"),
                value: false,
                flex: FlexParams {
                    flex: None,
                    alignment: None,
                },
                ..Default::default()
            }
        );
    }

    #[test]
    fn checkbox_widget() {
        let checkbox = Checkbox::new("Hello", false);

        Harness::run_test_window(checkbox, |harness| {
            let checkbox_state = harness.get_root_debug_state();
            assert_debug_snapshot!(checkbox_state);

            let new_checkbox = Checkbox::new("Hello", true);
            harness.update_root_element(new_checkbox);

            let checkbox_state_2 = harness.get_root_debug_state();
            assert_debug_snapshot!(checkbox_state_2);

            // TODO - Test reconcile() with text (currently not implemented for checkbox)
        });
    }

    // FIXME - Test currently doesn't work; checkbox doesn't emit events
    #[allow(dead_code)]
    fn checkbox_press() {
        use crate::elements::event_logger::EventLogger;
        use std::sync::mpsc::channel;

        let (event_sender, event_receiver) = channel();
        let checkbox_id = WidgetId::reserved(1);
        let checkbox = EventLogger::new(
            event_sender,
            Checkbox::new("Hello", false).with_reserved_id(checkbox_id),
        );

        Harness::run_test_window(checkbox, |harness| {
            assert_debug_snapshot!(harness.get_root_debug_state());

            harness.mouse_click_on(checkbox_id);

            let click_event = event_receiver.try_recv();
            assert_debug_snapshot!(click_event);

            // TODO - test data persistence, somehow?
        });
    }
}
