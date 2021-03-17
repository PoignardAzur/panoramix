use crate::glue::{Action, GlobalEventCx, Id};

use crate::element_tree::{Element, ElementExt, NoEvent, VirtualDom};
use crate::flex::FlexParams;
use crate::widgets::{CheckboxWidget, SingleCheckboxWidget};

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use tracing::{instrument, trace};

// TODO - Handle the anti-pattern where the user does something like
// Checkbox::new(false)
// In other words, enforce two-ways bindings

/// A checkbox with a text label.
///
/// ## Events
///
/// Emits [Toggled] events.
#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct Checkbox<CpEvent = NoEvent, CpState = ()> {
    pub text: String,
    pub value: bool,
    pub flex: FlexParams,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpEvent, CpState)>,
}

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct CheckboxData<CpEvent = NoEvent, CpState = ()> {
    pub text: String,
    pub value: bool,
    pub flex: FlexParams,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpEvent, CpState)>,
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

impl<CpEvent, CpState> Checkbox<CpEvent, CpState> {
    /// Build a checkbox with the given label.
    ///
    /// Use the [.on_toggled](Checkbox::on_toggled) method to provide a closure to be called when the box is toggled.
    pub fn new(text: impl Into<String>, value: bool) -> Self {
        Checkbox {
            text: text.into(),
            value,
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
            _markers: Default::default(),
        }
    }

    /// Change the way the checkbox's size is calculated
    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        Checkbox {
            flex: flex_params,
            ..self
        }
    }

    /// Provide a closure to be called when this checkbox is toggled.
    pub fn on_toggled(
        self,
        callback: impl Fn(&mut CpState, Toggled),
    ) -> impl Element<CpEvent, CpState> {
        self.on(callback)
    }
}

impl<CpEvent, CpState> Element<CpEvent, CpState> for Checkbox<CpEvent, CpState> {
    type Event = Toggled;
    type AggregateChildrenState = ();
    type BuildOutput = CheckboxData<CpEvent, CpState>;

    #[instrument(name = "Checkbox", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (CheckboxData<CpEvent, CpState>, ()) {
        (
            CheckboxData {
                text: self.text,
                value: self.value,
                flex: self.flex,
                _markers: Default::default(),
            },
            (),
        )
    }
}

impl<CpEvent, CpState> VirtualDom<CpEvent, CpState> for CheckboxData<CpEvent, CpState> {
    type Event = Toggled;
    type AggregateChildrenState = ();

    type TargetWidgetSeq = SingleCheckboxWidget;

    #[instrument(name = "Checkbox", skip(self))]
    fn init_tree(&self) -> SingleCheckboxWidget {
        SingleCheckboxWidget::new(
            CheckboxWidget::new(self.text.clone(), self.value, Id::new()),
            self.flex,
        )
    }

    #[instrument(name = "Checkbox", skip(self, other, widget, ctx))]
    fn reconcile(&self, other: &Self, widget: &mut SingleCheckboxWidget, ctx: &mut ReconcileCtx) {
        let checkbox_widget = widget.widget_mut();
        if self.text != other.text {
            checkbox_widget.pod.widget_mut().set_text(self.text.clone());
        }
        checkbox_widget.value = self.value;
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
            Some(Toggled { new_value })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
