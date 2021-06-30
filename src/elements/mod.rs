//! GUI elements that can be built in a component.

mod button;
mod checkbox;
mod component;
mod element_list;
mod empty;
mod flex_element;
mod label;
mod optional_element;
mod textbox;

pub mod any_element;

// TODO
mod with_mock_state;
#[doc(hidden)]
pub use with_mock_state::*;

pub mod element_tuple;
pub mod event_logger;
#[doc(hidden)]
pub mod with_event;

mod compute_diff;

pub use button::{Button, ButtonClick};
pub use checkbox::{Checkbox, Toggled};
pub use component::Component;
pub use element_list::ElementList;
pub use empty::EmptyElement;
pub use flex_element::Flex;
pub use label::Label;
pub use optional_element::*;
pub use textbox::{TextBox, TextChanged};

pub mod backend {
    pub use super::button::ButtonData;
    pub use super::checkbox::CheckboxData;
    pub use super::component::{ComponentHolder, ComponentOutput};
    pub use super::element_list::ElementListData;
    pub use super::element_tuple::ElementTupleData;
    pub use super::empty::EmptyElementData;
    pub use super::flex_element::FlexData;
    pub use super::label::LabelData;
    pub use super::textbox::TextBoxData;
    pub use super::with_event::WithEventTarget;

    pub use super::compute_diff::{compute_diff, ListMutation, ListMutationItem};
}
