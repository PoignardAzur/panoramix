use crate::ctx::ReconcileCtx;
use crate::element_tree::{Element, ElementExt, VirtualDom};
use crate::flex::FlexParams;
use crate::glue::{Action, GlobalEventCx, WidgetId};
use crate::metadata::{Metadata, NoState};
use crate::widgets::ButtonWidget;

use tracing::{instrument, trace};

/// A button with a text label.
///
/// ## Events
///
/// Emits [ButtonClick] events.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Button {
    pub text: String,
    pub flex: FlexParams,
    pub reserved_widget_id: Option<WidgetId>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ButtonData {
    pub text: String,
    pub flex: FlexParams,
    pub reserved_widget_id: Option<WidgetId>,
}

/// Event emitted when a [Button] is clicked.
///
/// Note: Might hold data like "mouse position" or "button id" future versions.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ButtonClick;

//
// --- IMPLS

impl Button {
    /// Build a button with the given label.
    ///
    /// Use the [.on_click](Button::on_click) method to provide a closure to be called when the button is clicked.
    pub fn new(text: impl Into<String>) -> Self {
        Button {
            text: text.into(),
            flex: FlexParams {
                flex: None,
                alignment: None,
            },
            reserved_widget_id: None,
        }
    }

    /// Change the way the button's size is calculated
    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        Button {
            flex: flex_params,
            ..self
        }
    }

    /// For unit tests only.
    ///
    /// The button created by this element always has the same id. If two widgets are created
    /// with the same id (for instance, because the same button is returned twice because of
    /// a copy-paste error), impredictable behavior may ensue.
    pub fn with_reserved_id(self, widget_id: WidgetId) -> Self {
        Button {
            reserved_widget_id: Some(widget_id),
            ..self
        }
    }

    /// Provide a closure to be called when this button is clicked.
    pub fn on_click<ComponentEvent: 'static, ComponentState: 'static>(
        self,
        md: Metadata<ComponentEvent, ComponentState>,
        callback: impl Fn(&mut ComponentState, ButtonClick) + Clone + 'static,
    ) -> impl Element {
        self.on(md, callback)
    }
}

impl Element for Button {
    type Event = ButtonClick;

    type ComponentState = NoState;
    type AggregateChildrenState = ();
    type BuildOutput = ButtonData;

    #[instrument(name = "Button", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (ButtonData, ()) {
        (
            ButtonData {
                text: self.text,
                flex: self.flex,
                reserved_widget_id: self.reserved_widget_id,
            },
            (),
        )
    }
}

impl VirtualDom for ButtonData {
    type Event = ButtonClick;
    type AggregateChildrenState = ();
    type TargetWidgetSeq = ButtonWidget;

    #[instrument(name = "Button", skip(self))]
    fn init_tree(&self) -> ButtonWidget {
        let id = self.reserved_widget_id.unwrap_or_else(WidgetId::next);
        ButtonWidget::new(self.text.clone(), self.flex, id)
    }

    #[instrument(name = "Button", skip(self, _prev_value, _widget, _ctx))]
    fn reconcile(&self, _prev_value: &Self, _widget: &mut ButtonWidget, _ctx: &mut ReconcileCtx) {
        //widget.set_text(self.text.clone());
    }

    #[instrument(name = "Button", skip(self, _children_state, widget, cx))]
    fn process_local_event(
        &self,
        _children_state: &mut Self::AggregateChildrenState,
        widget: &mut ButtonWidget,
        cx: &mut GlobalEventCx,
    ) -> Option<ButtonClick> {
        // FIXME - Rework event dispatching
        let id = widget.id();
        if let Some(Action::Clicked) = cx.app_data.dequeue_action(id) {
            trace!("Processed button press");
            Some(ButtonClick)
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

    #[test]
    fn new_button() {
        let button = Button::new("Hello");
        let (button_data, ()) = button.clone().build(());

        assert_debug_snapshot!(button);
        assert_debug_snapshot!(button_data);

        assert_eq!(
            button_data,
            ButtonData {
                text: String::from("Hello"),
                flex: FlexParams {
                    flex: None,
                    alignment: None,
                },
                ..Default::default()
            }
        );
    }

    #[test]
    fn button_widget() {
        let button = Button::new("Hello");

        Harness::run_test_window(button, |harness| {
            let button_state = harness.get_root_debug_state();
            assert_debug_snapshot!(button_state);

            // FIXME - Test reconcile() method (currently doesn't work)
        });
    }

    #[test]
    fn button_press() {
        use crate::elements::event_logger::EventLogger;
        use std::sync::mpsc::channel;

        let (event_sender, event_receiver) = channel();
        let button_id = WidgetId::reserved(1);
        let button = EventLogger::new(
            event_sender,
            Button::new("Hello").with_reserved_id(button_id),
        );

        Harness::run_test_window(button, |harness| {
            assert_debug_snapshot!(harness.get_root_debug_state());

            harness.mouse_click_on(button_id);

            let click_event = event_receiver.try_recv();
            assert_debug_snapshot!(click_event);
        });
    }
}
