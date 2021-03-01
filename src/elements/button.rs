use crate::glue::{Action, GlobalEventCx, Id};

use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::widgets::flex::FlexParams;
use crate::widgets::ButtonWidget;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use tracing::{instrument, trace};

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct Button<CpState = (), CpEvent = NoEvent> {
    pub text: String,
    pub flex: FlexParams,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpState, CpEvent)>,
}

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct ButtonData<CpState = (), CpEvent = NoEvent> {
    pub text: String,
    pub flex: FlexParams,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpState, CpEvent)>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ButtonPressed;

//
// --- IMPLS

impl<CpState, CpEvent> Button<CpState, CpEvent> {
    pub fn new(text: impl Into<String>) -> Self {
        Button {
            text: text.into(),
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
            _markers: Default::default(),
        }
    }

    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        Button {
            flex: flex_params,
            ..self
        }
    }
}

impl<CpState, CpEvent> Element<CpState, CpEvent> for Button<CpState, CpEvent> {
    type Event = ButtonPressed;
    type AggregateChildrenState = ();
    type BuildOutput = ButtonData<CpState, CpEvent>;

    #[instrument(name = "Button", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (ButtonData<CpState, CpEvent>, ()) {
        (
            ButtonData {
                text: self.text,
                flex: self.flex,
                _markers: Default::default(),
            },
            (),
        )
    }
}

impl<CpState, CpEvent> VirtualDom<CpState, CpEvent> for ButtonData<CpState, CpEvent> {
    type Event = ButtonPressed;
    type AggregateChildrenState = ();
    type TargetWidgetSeq = ButtonWidget;

    #[instrument(name = "Button", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Button", skip(self))]
    fn init_tree(&self) -> ButtonWidget {
        ButtonWidget::new(self.text.clone(), self.flex, Id::new())
    }

    #[instrument(name = "Button", skip(self, _other, _widget, _ctx))]
    fn reconcile(&self, _other: &Self, _widget: &mut ButtonWidget, _ctx: &mut ReconcileCtx) {
        //widget.set_text(self.text.clone());
    }

    #[instrument(
        name = "Button",
        skip(self, _component_state, _children_state, widget, cx)
    )]
    fn process_local_event(
        &self,
        _component_state: &mut CpState,
        _children_state: &mut Self::AggregateChildrenState,
        widget: &mut ButtonWidget,
        cx: &mut GlobalEventCx,
    ) -> Option<ButtonPressed> {
        // FIXME - Rework event dispatching
        let id = widget.id;
        if let Some(Action::Clicked) = cx.app_data.dequeue_action(id) {
            trace!("Processed button press");
            Some(ButtonPressed)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element_tree::assign_empty_state_type;
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
                    flex: 1.0,
                    alignment: None,
                },
                ..Default::default()
            }
        );

        assign_empty_state_type(&button);
    }

    // TODO
    // - Id test (??)
    // - Event test
    // - Widget test
}
