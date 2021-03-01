use crate::glue::{Action, GlobalEventCx, Id};

use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::widgets::flex::FlexParams;
use crate::widgets::TextBoxWidget;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use tracing::{instrument, trace};

// TODO - Handle the anti-pattern where the user does something like
// TextBox::new("Some fixed string")
// In other words, enforce two-ways bindings

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct TextBox<CpState = (), CpEvent = NoEvent>(
    pub String,
    pub FlexParams,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct TextBoxData<CpState = (), CpEvent = NoEvent>(
    pub String,
    pub FlexParams,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextChanged(pub String);

//
// --- IMPLS

impl<CpState, CpEvent> TextBox<CpState, CpEvent> {
    pub fn new(text: impl Into<String>) -> Self {
        TextBox(
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
        TextBox(self.0, flex_params, Default::default(), Default::default())
    }
}

impl<CpState, CpEvent> Element<CpState, CpEvent> for TextBox<CpState, CpEvent> {
    type Event = TextChanged;
    type AggregateChildrenState = ();
    type BuildOutput = TextBoxData<CpState, CpEvent>;

    #[instrument(name = "TextBox", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (TextBoxData<CpState, CpEvent>, ()) {
        (
            TextBoxData(self.0, self.1, Default::default(), Default::default()),
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
        let text = &self.0;
        TextBoxWidget::new(text.clone(), self.1, Id::new())
    }

    #[instrument(name = "TextBox", skip(self, _other, widget, ctx))]
    fn reconcile(&self, _other: &Self, widget: &mut Self::TargetWidgetSeq, ctx: &mut ReconcileCtx) {
        let text = &self.0;
        widget.text = text.clone();
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
        widget: &mut Self::TargetWidgetSeq,
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
