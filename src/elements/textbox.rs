use crate::glue::{Action, GlobalEventCx, WidgetId};

use crate::element_tree::{Element, ElementExt, NoEvent, VirtualDom};
use crate::flex::FlexParams;
use crate::widgets::TextBoxWidget;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
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
#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct TextBox<CpEvent = NoEvent, CpState = ()> {
    pub text: String,
    pub flex: FlexParams,
    pub reserved_widget_id: Option<WidgetId>,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpEvent, CpState)>,
}

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct TextBoxData<CpEvent = NoEvent, CpState = ()> {
    pub text: String,
    pub flex: FlexParams,
    pub reserved_widget_id: Option<WidgetId>,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpEvent, CpState)>,
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

impl<CpEvent, CpState> TextBox<CpEvent, CpState> {
    /// Build a text box with the given content.
    ///
    /// Use the [.on_text_changed](TextBox::on_text_changed) method to provide a closure to be called when the box is edited.
    pub fn new(text: impl Into<String>) -> Self {
        TextBox {
            text: text.into(),
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
            reserved_widget_id: None,
            _markers: Default::default(),
        }
    }

    /// Change the way the box's size is calculated
    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        TextBox {
            flex: flex_params,
            ..self
        }
    }

    pub fn with_reserved_id(self, widget_id: WidgetId) -> Self {
        TextBox {
            reserved_widget_id: Some(widget_id),
            ..self
        }
    }

    /// Provide a closure to be called when this box is edited.
    pub fn on_text_changed(
        self,
        callback: impl Fn(&mut CpState, TextChanged) + Clone,
    ) -> impl Element<CpEvent, CpState> {
        self.on(callback)
    }
}

impl<CpEvent, CpState> Element<CpEvent, CpState> for TextBox<CpEvent, CpState> {
    type Event = TextChanged;
    type AggregateChildrenState = ();
    type BuildOutput = TextBoxData<CpEvent, CpState>;

    #[instrument(name = "TextBox", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (TextBoxData<CpEvent, CpState>, ()) {
        (
            TextBoxData {
                text: self.text,
                flex: self.flex,
                reserved_widget_id: self.reserved_widget_id,
                _markers: Default::default(),
            },
            (),
        )
    }
}

impl<CpEvent, CpState> VirtualDom<CpEvent, CpState> for TextBoxData<CpEvent, CpState> {
    type Event = TextChanged;
    type AggregateChildrenState = ();

    type TargetWidgetSeq = TextBoxWidget;

    #[instrument(name = "TextBox", skip(self))]
    fn init_tree(&self) -> TextBoxWidget {
        let id = self.reserved_widget_id.unwrap_or_else(WidgetId::next);
        TextBoxWidget::new(self.text.clone(), self.flex, id)
    }

    #[instrument(name = "TextBox", skip(self, _other, widget, ctx))]
    fn reconcile(&self, _other: &Self, widget: &mut TextBoxWidget, ctx: &mut ReconcileCtx) {
        widget.text = self.text.clone();
        // TODO - check diff with previous value
        widget.request_druid_update(ctx);
    }

    #[instrument(
        name = "TextBox",
        skip(self, _component_state, _children_state, widget, cx)
    )]
    fn process_local_event(
        &self,
        _component_state: &mut CpState,
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
    use crate::element_tree::assign_empty_state_type;
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
                    flex: 1.0,
                    alignment: None,
                },
                ..Default::default()
            }
        );

        assign_empty_state_type(&textbox);
    }

    #[test]
    fn textbox_widget() {
        // TODO - We use Tuple! because RootWidget currently wants a root element with no event
        use crate::Tuple;
        let textbox = Tuple!(TextBox::new("Hello"));

        Harness::run_test_window(textbox, |harness| {
            let textbox_state = harness.get_root_debug_state();
            assert_debug_snapshot!(textbox_state);

            let new_textbox = Tuple!(TextBox::new("World"));
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
