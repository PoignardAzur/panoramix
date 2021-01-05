use crate::glue::{DruidAppData, GlobalEventCx, Id};

use crate::element_tree::{ElementTree, VirtualDom};
use crate::widgets::{make_button, SingleWidget};

use druid::widget as druid_w;
use druid::widget::Click;
use druid::widget::ControllerHost;

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Button<ExplicitState>(pub ButtonData<ExplicitState>);

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ButtonData<ParentComponentState>(
    pub String,
    pub std::marker::PhantomData<ParentComponentState>,
);

//
// --- IMPLS

impl<ExplicitState> Button<ExplicitState> {
    pub fn new(text: impl Into<String>) -> Button<ExplicitState> {
        Button(ButtonData(text.into(), Default::default()))
    }
}

impl<ExplicitState> ElementTree<ExplicitState> for Button<ExplicitState> {
    type Event = ButtonPressed;
    type AggregateComponentState = ();
    type BuildOutput = ButtonData<ExplicitState>;

    fn build(self, _prev_state: ()) -> (ButtonData<ExplicitState>, ()) {
        (self.0, ())
    }
}

pub struct ButtonPressed();

impl<ParentComponentState> VirtualDom<ParentComponentState> for ButtonData<ParentComponentState> {
    type Event = ButtonPressed;
    type DomState = Id;
    type AggregateComponentState = ();

    // FIXME
    type TargetWidgetSeq =
        SingleWidget<ControllerHost<druid_w::Button<DruidAppData>, Click<DruidAppData>>>;

    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    fn init_tree(&self) -> (Self::TargetWidgetSeq, Id) {
        let text = &self.0;
        let id = Id::new();
        (make_button(text.clone(), id), id)
    }

    fn apply_diff(
        &self,
        _other: &Self,
        prev_state: Self::DomState,
        _widget: &mut Self::TargetWidgetSeq,
    ) -> Id {
        let _text = &self.0;
        //widget.set_text(text.clone());
        prev_state
    }

    fn process_event(
        &self,
        _explicit_state: &mut ParentComponentState,
        _children_state: &mut (),
        dom_state: &mut Id,
        _cx: &mut GlobalEventCx,
    ) -> Option<ButtonPressed> {
        let id = *dom_state;
        if _cx.app_data.dequeue_action(id).is_some() {
            Some(ButtonPressed())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_button() {
        let button = Button::<()>::new("Hello");
        let (button_data, _) = button.clone().build(());
        assert_eq!(
            button,
            Button(ButtonData(String::from("Hello"), Default::default()))
        );
        assert_eq!(
            button_data,
            ButtonData(String::from("Hello"), Default::default())
        );
    }

    // TODO
    // - Id test (??)
    // - Event test
    // - Widget test
}
