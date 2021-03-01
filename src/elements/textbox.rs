use crate::glue::{Action, GlobalEventCx, Id};

use crate::element_tree::{Element, ElementExt, NoEvent, VirtualDom};
use crate::widgets::flex::FlexParams;
use crate::widgets::TextBoxWidget;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use tracing::{instrument, trace};

// TODO - Handle the anti-pattern where the user does something like
// TextBox::new("Some fixed string")
// In other words, enforce two-ways bindings

// TODO - Add "validate on enter" feature

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct TextBox<CpState = (), CpEvent = NoEvent> {
    pub text: String,
    pub flex: FlexParams,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpState, CpEvent)>,
}

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct TextBoxData<CpState = (), CpEvent = NoEvent> {
    pub text: String,
    pub flex: FlexParams,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpState, CpEvent)>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextChanged(pub String);

//
// --- IMPLS

impl<CpState, CpEvent> TextBox<CpState, CpEvent> {
    pub fn new(text: impl Into<String>) -> Self {
        TextBox {
            text: text.into(),
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
            _markers: Default::default(),
        }
    }

    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        TextBox {
            flex: flex_params,
            ..self
        }
    }

    pub fn on_text_changed(
        self,
        callback: impl Fn(&mut CpState, TextChanged),
    ) -> impl Element<CpState, CpEvent> {
        self.on(callback)
    }
}

impl<CpState, CpEvent> Element<CpState, CpEvent> for TextBox<CpState, CpEvent> {
    type Event = TextChanged;
    type AggregateChildrenState = ();
    type BuildOutput = TextBoxData<CpState, CpEvent>;

    #[instrument(name = "TextBox", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (TextBoxData<CpState, CpEvent>, ()) {
        (
            TextBoxData {
                text: self.text,
                flex: self.flex,
                _markers: Default::default(),
            },
            (),
        )
    }
}

impl<CpState, CpEvent> VirtualDom<CpState, CpEvent> for TextBoxData<CpState, CpEvent> {
    type Event = TextChanged;
    type AggregateChildrenState = ();

    type TargetWidgetSeq = TextBoxWidget;

    #[instrument(name = "TextBox", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "TextBox", skip(self))]
    fn init_tree(&self) -> TextBoxWidget {
        TextBoxWidget::new(self.text.clone(), self.flex, Id::new())
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
        let id = widget.id;
        if let Some(Action::TextChanged(new_text)) = cx.app_data.dequeue_action(id) {
            trace!("Processed text change");
            Some(TextChanged(new_text))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
