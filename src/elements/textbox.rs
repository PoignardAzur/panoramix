use crate::ctx::ReconcileCtx;
use crate::element_tree::{Element, ElementExt, VirtualDom};
use crate::flex::FlexParams;
use crate::glue::{Action, GlobalEventCx, WidgetId};
use crate::metadata::{Metadata, NoState};
use crate::widgets::TextBoxWidget;

use tracing::{instrument, trace};

// TODO - Handle the anti-pattern where the user does something like
// TextBox::new("Some fixed string")
// In other words, enforce two-ways bindings

// TODO - Add "validate on enter" feature

/// A text-editing box.
///
/// ## Events
///
/// Emits [TextChanged] events.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextBox {
    pub text: String,
    pub flex: FlexParams,
    pub reserved_widget_id: Option<WidgetId>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextBoxData {
    pub text: String,
    pub flex: FlexParams,
    pub reserved_widget_id: Option<WidgetId>,
}

/// Event emitted when text is entered or edited in a [TextBox].
///
/// Holds the new content of the box
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextChanged {
    pub new_content: String,
}

//
// --- IMPLS

impl TextBox {
    /// Build a text box with the given content.
    ///
    /// Use the [.on_text_changed](TextBox::on_text_changed) method to provide a closure to be called when the box is edited.
    pub fn new(text: impl Into<String>) -> Self {
        TextBox {
            text: text.into(),
            flex: FlexParams {
                flex: None,
                alignment: None,
            },
            reserved_widget_id: None,
        }
    }

    /// Change the way the box's size is calculated
    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        TextBox {
            flex: flex_params,
            ..self
        }
    }

    /// For unit tests only.
    ///
    /// The textbox created by this element always has the same id. If two widgets are created
    /// with the same id (for instance, because the same textbox is returned twice because of
    /// a copy-paste error), impredictable behavior may ensue.
    pub fn with_reserved_id(self, widget_id: WidgetId) -> Self {
        TextBox {
            reserved_widget_id: Some(widget_id),
            ..self
        }
    }

    /// Provide a closure to be called when this box is edited.
    pub fn on_text_changed<ComponentEvent: 'static, ComponentState: 'static>(
        self,
        md: Metadata<ComponentEvent, ComponentState>,
        callback: impl Fn(&mut ComponentState, TextChanged) + Clone + 'static,
    ) -> impl Element {
        self.on(md, callback)
    }
}

impl Element for TextBox {
    type Event = TextChanged;

    type ComponentState = NoState;
    type AggregateChildrenState = ();
    type BuildOutput = TextBoxData;

    #[instrument(name = "TextBox", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (TextBoxData, ()) {
        (
            TextBoxData {
                text: self.text,
                flex: self.flex,
                reserved_widget_id: self.reserved_widget_id,
            },
            (),
        )
    }
}

impl VirtualDom for TextBoxData {
    type Event = TextChanged;
    type AggregateChildrenState = ();

    type TargetWidgetSeq = TextBoxWidget;

    #[instrument(name = "TextBox", skip(self))]
    fn init_tree(&self) -> TextBoxWidget {
        let id = self.reserved_widget_id.unwrap_or_else(WidgetId::next);
        TextBoxWidget::new(self.text.clone(), self.flex, id)
    }

    #[instrument(name = "TextBox", skip(self, _prev_value, widget, ctx))]
    fn reconcile(&self, _prev_value: &Self, widget: &mut TextBoxWidget, ctx: &mut ReconcileCtx) {
        widget.text = self.text.clone();
        // TODO - check diff with previous value
        widget.request_druid_update(ctx.event_ctx);
    }

    #[instrument(name = "TextBox", skip(self, _children_state, widget, cx))]
    fn process_local_event(
        &self,
        _children_state: &mut Self::AggregateChildrenState,
        widget: &mut TextBoxWidget,
        cx: &mut GlobalEventCx,
    ) -> Option<TextChanged> {
        // FIXME - Rework event dispatching
        let id = widget.id();
        if let Some(Action::TextChanged(new_content)) = cx.app_data.dequeue_action(id) {
            trace!("Processed text change");
            Some(TextChanged { new_content })
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
    fn new_textbox() {
        let textbox = TextBox::new("Hello");
        let (textbox_data, ()) = textbox.clone().build(());

        assert_debug_snapshot!(textbox);
        assert_debug_snapshot!(textbox_data);

        assert_eq!(
            textbox_data,
            TextBoxData {
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
    fn textbox_widget() {
        let textbox = TextBox::new("Hello");

        Harness::run_test_window(textbox, |harness| {
            let textbox_state = harness.get_root_debug_state();
            assert_debug_snapshot!(textbox_state);

            let new_textbox = TextBox::new("World");
            harness.update_root_element(new_textbox);

            let textbox_state_2 = harness.get_root_debug_state();
            assert_debug_snapshot!(textbox_state_2);
        });
    }

    #[test]
    fn textbox_keydown() {
        use crate::elements::event_logger::EventLogger;
        use std::sync::mpsc::channel;

        let (event_sender, event_receiver) = channel();
        let textbox_id = WidgetId::reserved(42);
        let textbox = EventLogger::new(
            event_sender,
            TextBox::new("Hello").with_reserved_id(textbox_id),
        );

        Harness::run_test_window(textbox, |harness| {
            assert_debug_snapshot!(harness.get_root_debug_state());

            harness.mouse_click_on(textbox_id);
            harness.keyboard_key("a");

            let text_event = event_receiver.try_recv();
            assert!(matches!(text_event, Ok(TextChanged { .. })));
            // TODO - Because we don't mock IME events, the emitted event doesn't
            // have the right text
            //assert_debug_snapshot!(click_event);

            // TODO - test data persistence, somehow?
        });
    }
}
