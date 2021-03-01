use crate::glue::{Action, GlobalEventCx, Id};

use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::widgets::flex::FlexParams;
use crate::widgets::{CheckboxWidget, SingleCheckboxWidget};

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use tracing::{instrument, trace};

// TODO - Handle the anti-pattern where the user does something like
// Checkbox::new(false)
// In other words, enforce two-ways bindings

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct Checkbox<CpState = (), CpEvent = NoEvent>(
    pub String,
    pub bool,
    pub FlexParams,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct CheckboxData<CpState = (), CpEvent = NoEvent>(
    pub String,
    pub bool,
    pub FlexParams,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Toggled(pub bool);

//
// --- IMPLS

impl<CpState, CpEvent> Checkbox<CpState, CpEvent> {
    pub fn new(text: impl Into<String>, value: bool) -> Self {
        Checkbox(
            text.into(),
            value,
            FlexParams {
                flex: 1.0,
                alignment: None,
            },
            Default::default(),
            Default::default(),
        )
    }

    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        Checkbox(
            self.0,
            self.1,
            flex_params,
            Default::default(),
            Default::default(),
        )
    }
}

impl<CpState, CpEvent> Element<CpState, CpEvent> for Checkbox<CpState, CpEvent> {
    type Event = Toggled;
    type AggregateChildrenState = ();
    type BuildOutput = CheckboxData<CpState, CpEvent>;

    #[instrument(name = "Checkbox", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (CheckboxData<CpState, CpEvent>, ()) {
        (
            CheckboxData(
                self.0,
                self.1,
                self.2,
                Default::default(),
                Default::default(),
            ),
            (),
        )
    }
}

impl<CpState, CpEvent> VirtualDom<CpState, CpEvent> for CheckboxData<CpState, CpEvent> {
    type Event = Toggled;
    type AggregateChildrenState = ();

    type TargetWidgetSeq = SingleCheckboxWidget;

    #[instrument(name = "Checkbox", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Checkbox", skip(self))]
    fn init_tree(&self) -> SingleCheckboxWidget {
        let text = &self.0;
        let checkbox =
            SingleCheckboxWidget::new(CheckboxWidget::new(text.clone(), self.1, Id::new()), self.2);
        checkbox
    }

    #[instrument(name = "Checkbox", skip(self, other, widget, ctx))]
    fn reconcile(&self, other: &Self, widget: &mut SingleCheckboxWidget, ctx: &mut ReconcileCtx) {
        let checkbox_widget = widget.widget_mut();
        let text = &self.0;
        let prev_text = &other.0;
        if text != prev_text {
            checkbox_widget.pod.widget_mut().set_text(text.clone());
        }
        checkbox_widget.value = self.1;
        // TODO - check diff with previous value
        widget.request_druid_update(ctx);
        widget.widget_mut().request_druid_update(ctx);
    }

    #[instrument(
        name = "Checkbox",
        skip(self, _component_state, _children_state, widget, cx)
    )]
    fn process_local_event(
        &self,
        _component_state: &mut CpState,
        _children_state: &mut Self::AggregateChildrenState,
        widget: &mut SingleCheckboxWidget,
        cx: &mut GlobalEventCx,
    ) -> Option<Toggled> {
        // FIXME - Rework event dispatching
        let id = widget.widget().id;
        if let Some(Action::Clicked) = cx.app_data.dequeue_action(id) {
            let new_value = widget.widget().value;
            trace!("Processed checkbox toggle: {}", new_value);
            Some(Toggled(new_value))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
