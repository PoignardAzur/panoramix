//! Panoramix is an experimental GUI framework for the Rust programming language.
//!
//! This framework is **data-driven and declarative**, drawing some inspiration from [React](https://github.com/facebook/react), and implemented on top of the [Druid](https://github.com/linebender/druid) toolkit.
//!
//! It aims to use **simple, idiomatic Rust**: Panoramix doesn't use unsafe code, cells, mutexes, or DSL macros.
//!
//!
//! ## Getting started
//!
//! Here is our "hello world" example:
//!
//! ```no_run
//! use panoramix::elements::{Button, Label};
//! use panoramix::{component, Column, CompCtx, Element, Metadata, NoEvent, RootHandler};
//!
//! #[component]
//! fn HelloBox(ctx: &CompCtx, _props: ()) -> impl Element<Event = NoEvent> {
//!     let md = ctx.use_metadata::<NoEvent, ()>();
//!     Column!(
//!         Label::new("Hello world!"),
//!         Button::new("Say hello").on_click(md, |_, _| {
//!             println!("Hello world");
//!         })
//!     )
//! }
//!
//! fn main() -> Result<(), panoramix::PlatformError> {
//!     RootHandler::new(HelloBox)
//!         .with_tracing(true)
//!         .launch()
//! }
//! ```
//!
//! To understand this example, let's define a few terms:
//!
//! - A **Widget** is the fundamental unit of GUI; for instance, a text field and a label are both widgets. You've probably seen the term if you've used other GUI frameworks.
//! - An **Element** is a lightweight description of a Widget. In our example, [`Button::new`](elements::Button::new) and [`Label::new`](elements::Label::new) both return elements. The [`Column`] macro is similar to `vec![]` - it takes an arbitrary number of elements and returns a container element.
//!   - Some elements have builder methods, that return new elements. Eg: [`Button::on_click`](elements::Button::on_click).
//! - A **Component** is a user-written function that takes **Props** as parameters and returns a tree of elements (or, more accurately, an arbitrary element that may or may not contain other elements). In our example, `HelloBox` is a component. By convention, components always have a CamelCase name.
//!
//! In Panoramix, you don't directly manipulate **widgets**; instead, you write **components** that return **elements**. The framework calls your components, gets a tree of elements, and builds a matching widget tree for you. When some event changes the application state, the framework calls your components again, gets a new element tree, and edits the widget tree accordingly.
//!
//! As such, the root of your Panoramix application will usually look like:
//!
//! ```no_run
//! // main.rs
//!
//! use panoramix::elements::{Button, ButtonClick};
//! use panoramix::{component, CompCtx, Element, NoEvent, RootHandler};
//!
//! #[derive(Debug, Default, Clone, PartialEq)]
//! struct ApplicationState {
//!     // ...
//! }
//!
//! #[component]
//! fn MyRootComponent(ctx: &CompCtx, _props: ()) -> impl Element<Event = NoEvent> {
//!     // ...
//!     # panoramix::elements::EmptyElement::new()
//! }
//!
//! fn main() -> Result<(), panoramix::PlatformError> {
//!     RootHandler::new(MyRootComponent)
//!         .with_tracing(true)
//!         .launch()
//! }
//! ```
//!
//! For information on how to write a component, see [these tutorials](tutorials).

mod ctx;
mod element_tree;
mod glue;
mod metadata;
mod root_handler;
mod widget_sequence;

pub mod test_harness;

pub mod elements;
pub mod widgets;

pub mod flex;

pub use panoramix_derive::component;

pub use crate::ctx::CompCtx;
pub use element_tree::{Element, ElementExt};
pub use metadata::{Metadata, NoEvent};
pub use root_handler::{PlatformError, RootHandler, RootWidget};

/// Traits and type used internally to compute the GUI.
///
/// The end user should never manipulate these types directly; but someone wanting to
/// create a custom element might need to.
pub mod internals {
    // Note: These items are declared together in files, but are exported separately here
    // to have a clean separation in the documentation between the items required to write
    // a GUI and the items required to create a GUI element.

    pub use crate::ctx::{ProcessEventCtx, ReconcileCtx};
    pub use crate::element_tree::VirtualDom;
    pub use crate::glue::{Action, DruidAppData, GlobalEventCx, WidgetId};
    pub use crate::widget_sequence::{FlexWidget, WidgetSequence};
}

/// Dummy modules, with tutorials integrated to the doc.
pub mod tutorials {
    #[doc = include_str!("../tutorials/01_writing_a_component.md")]
    pub mod t_01_writing_a_component {}
    #[doc = include_str!("../tutorials/02_event_handling.md")]
    pub mod t_02_event_handling {}
    #[doc = include_str!("../tutorials/03_local_state.md")]
    pub mod t_03_local_state {}

    #[doc = include_str!("../tutorials/speedrun_tutorial.md")]
    pub mod t_speedrun_tutorial {}
}
