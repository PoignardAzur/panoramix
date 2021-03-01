use crate::glue::{Action, GlobalEventCx, Id};

use crate::element_tree::{Element, ElementExt, NoEvent, VirtualDom};
use crate::widgets::flex::FlexParams;
use crate::widgets::ButtonWidget;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use tracing::{instrument, trace};

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct Button<CpEvent = NoEvent, CpState = ()> {
    pub text: String,
    pub flex: FlexParams,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpEvent, CpState)>,
}

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct ButtonData<CpEvent = NoEvent, CpState = ()> {
    pub text: String,
    pub flex: FlexParams,
    #[derivative(Debug = "ignore")]
    pub _markers: std::marker::PhantomData<(CpEvent, CpState)>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ButtonClick;

//
// --- IMPLS

impl<CpEvent, CpState> Button<CpEvent, CpState> {
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

    pub fn on_click(
        self,
        callback: impl Fn(&mut CpState, ButtonClick),
    ) -> impl Element<CpEvent, CpState> {
        self.on(callback)
    }
}

impl<CpEvent, CpState> Element<CpEvent, CpState> for Button<CpEvent, CpState> {
    type Event = ButtonClick;
    type AggregateChildrenState = ();
    type BuildOutput = ButtonData<CpEvent, CpState>;

    #[instrument(name = "Button", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (ButtonData<CpEvent, CpState>, ()) {
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

impl<CpEvent, CpState> VirtualDom<CpEvent, CpState> for ButtonData<CpEvent, CpState> {
    type Event = ButtonClick;
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
    ) -> Option<ButtonClick> {
        // FIXME - Rework event dispatching
        let id = widget.id;
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
