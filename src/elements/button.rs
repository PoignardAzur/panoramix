use crate::glue::{GlobalEventCx, Id};

use crate::element_tree::{ElementTree, NoEvent, VirtualDom};
use crate::widgets::ButtonWidget;

use derivative::Derivative;
use tracing::instrument;

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct Button<ComponentState = (), ComponentEvent = NoEvent>(
    pub String,
    pub std::marker::PhantomData<ComponentState>,
    pub std::marker::PhantomData<ComponentEvent>,
);

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct ButtonData<ComponentState = (), ComponentEvent = NoEvent>(
    pub String,
    pub std::marker::PhantomData<ComponentState>,
    pub std::marker::PhantomData<ComponentEvent>,
);

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ButtonPressed();

//
// --- IMPLS

impl<ComponentState, ComponentEvent> Button<ComponentState, ComponentEvent> {
    pub fn new(text: impl Into<String>) -> Self {
        Button(text.into(), Default::default(), Default::default())
    }
}

impl<ComponentState, ComponentEvent> ElementTree<ComponentState, ComponentEvent>
    for Button<ComponentState, ComponentEvent>
{
    type Event = ButtonPressed;
    type AggregateChildrenState = ();
    type BuildOutput = ButtonData<ComponentState, ComponentEvent>;

    #[instrument(name = "Button", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (ButtonData<ComponentState, ComponentEvent>, ()) {
        (
            ButtonData(self.0, Default::default(), Default::default()),
            (),
        )
    }
}

impl<ComponentState, ComponentEvent> VirtualDom<ComponentState, ComponentEvent>
    for ButtonData<ComponentState, ComponentEvent>
{
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
        ButtonWidget::new(text.clone(), Id::new())
    }

    #[instrument(name = "Button", skip(self, _other, _widget))]
    fn reconcile(&self, _other: &Self, _widget: &mut Self::TargetWidgetSeq) {
        let _text = &self.0;
        //widget.set_text(text.clone());
    }

    #[instrument(
        name = "Button",
        skip(self, _component_state, _children_state, widget, cx)
    )]
    fn process_event(
        &self,
        _component_state: &mut ComponentState,
        _children_state: &mut (),
        widget: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<ButtonPressed> {
        // FIXME - Rework event dispatching
        let id = widget.1;
        if cx.app_data.dequeue_action(id).is_some() {
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
                Default::default(),
                Default::default()
            )
        );

        assign_empty_state_type(&button);
    }

    // TODO
    // - Id test (??)
    // - Event test
    // - Widget test
}
