use crate::glue::{GlobalEventCx, Id};

use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::widgets::flex::FlexParams;
use crate::widgets::ButtonWidget;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use tracing::{instrument, trace};

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct Button<CpState = (), CpEvent = NoEvent>(
    pub String,
    pub FlexParams,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Derivative, PartialEq)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct ButtonData<CpState = (), CpEvent = NoEvent>(
    pub String,
    pub FlexParams,
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ButtonPressed();

//
// --- IMPLS

impl<CpState, CpEvent> Button<CpState, CpEvent> {
    pub fn new(text: impl Into<String>) -> Self {
        Button(
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
        Button(self.0, flex_params, Default::default(), Default::default())
    }
}

impl<CpState, CpEvent> Element<CpState, CpEvent> for Button<CpState, CpEvent> {
    type Event = ButtonPressed;
    type AggregateChildrenState = ();
    type BuildOutput = ButtonData<CpState, CpEvent>;

    #[instrument(name = "Button", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (ButtonData<CpState, CpEvent>, ()) {
        (
            ButtonData(self.0, self.1, Default::default(), Default::default()),
            (),
        )
    }
}

impl<CpState, CpEvent> VirtualDom<CpState, CpEvent> for ButtonData<CpState, CpEvent> {
    type Event = ButtonPressed;
    type AggregateChildrenState = ();

    // FIXME
    type TargetWidgetSeq = ButtonWidget;

    #[instrument(name = "Button", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Button", skip(self))]
    fn init_tree(&self) -> ButtonWidget {
        let text = &self.0;
        ButtonWidget::new(text.clone(), self.1, Id::new())
    }

    #[instrument(name = "Button", skip(self, _other, _widget, _ctx))]
    fn reconcile(
        &self,
        _other: &Self,
        _widget: &mut Self::TargetWidgetSeq,
        _ctx: &mut ReconcileCtx,
    ) {
        let _text = &self.0;
        //widget.set_text(text.clone());
    }

    #[instrument(
        name = "Button",
        skip(self, _component_state, _children_state, widget, cx)
    )]
    fn process_local_event(
        &self,
        _component_state: &mut CpState,
        _children_state: &mut Self::AggregateChildrenState,
        widget: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<ButtonPressed> {
        // FIXME - Rework event dispatching
        let id = widget.id;
        if cx.app_data.dequeue_action(id).is_some() {
            trace!("Processed button press");
            Some(ButtonPressed())
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
            ButtonData(
                String::from("Hello"),
                FlexParams {
                    flex: 1.0,
                    alignment: None,
                },
                Default::default(),
                Default::default(),
            )
        );

        assign_empty_state_type(&button);
    }

    // TODO
    // - Id test (??)
    // - Event test
    // - Widget test
}
