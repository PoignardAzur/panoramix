//! GUI elements that can be built in a component.

mod any_element;
mod button;
mod checkbox;
mod component;
mod element_list;
mod element_option;
mod empty;
mod flex_element;
mod label;
mod textbox;

pub mod element_tuple;
mod event_logger;
mod mock_component;
mod with_event;

mod compute_diff;

pub use any_element::ElementBox;
pub use button::{Button, ButtonClick};
pub use checkbox::{Checkbox, Toggled};
pub use component::{Component, ComponentOutput};
pub use element_list::ElementList;
pub use element_option::*;
pub use empty::EmptyElement;
pub use flex_element::Flex;
pub use label::Label;
pub use textbox::{TextBox, TextChanged};

// TODO - doc
pub mod internals {
    pub use super::any_element::VirtualDomBox;
    pub use super::button::ButtonData;
    pub use super::checkbox::CheckboxData;
    pub use super::component::{ComponentHolder, ComponentOutputData};
    pub use super::element_list::ElementListData;
    pub use super::element_tuple::ElementTupleData;
    pub use super::empty::EmptyElementData;
    pub use super::flex_element::FlexData;
    pub use super::label::LabelData;
    pub use super::textbox::TextBoxData;
    pub use super::with_event::WithEventTarget;
    pub use super::with_event::{ParentEvent, WithBubbleEvent, WithCallbackEvent, WithMapEvent};

    // TODO - move to test_harness?
    pub use super::event_logger::{EventLogger, EventLoggerData};
    pub use super::mock_component::{MockComponent, MockComponentData, MockState};

    pub use super::compute_diff::{compute_diff, ListMutation, ListMutationItem};
}
