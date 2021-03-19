**Note: this is the same text as the documentation in `lib.rs`**

---

Panoramix is an experimental GUI framework for the Rust programming language.

This framework is **data-driven and declarative**, drawing some inspiration from [React](https://github.com/facebook/react), and implemented on top of the [Druid](https://github.com/linebender/druid) toolkit.

It aims to use **simple, idiomatic Rust**: Panoramix doesn't use unsafe code, cells, mutexes, or DSL macros.


## Getting started

Here is our "hello world" example:

```no_run
use panoramix::elements::{Button, Label};
use panoramix::{component, Column, CompCtx, Element, NoEvent, RootHandler};

#[component]
fn HelloBox(_ctx: &CompCtx, _props: ()) -> impl Element {
    Column!(
        Label::new("Hello world!"),
        Button::new("Say hello").on_click(|_, _| {
            println!("Hello world");
        })
    )
}

fn main() -> Result<(), panoramix::PlatformError> {
    RootHandler::new(HelloBox::new(()))
        .with_tracing(true)
        .launch()
}
```

To understand this example, let's define a few terms:

- A **Widget** is the fundamental unit of GUI; for instance, a text field and a label are both widgets. You've probably seen the term if you've used other GUI frameworks.
- An **Element** is a lightweight description of a Widget. In our example, [`Button::new`](elements::Button::new) and [`Label::new`](elements::Label::new) both return elements. The [`Column`] macro is similar to `vec` - it takes an arbitrary number of elements and returns a container element.
  - Some elements have builder methods. Eg: [`Button::on_click`](elements::Button::on_click).
- A **Component** is a user-written function that takes **Props** as parameters and returns a tree of elements (or, more accurately, an arbitrary element that may or may not contain other elements). In our example, `HelloBox` is a component. By convention, components always have a CamelCase name.

In Panoramix, you don't directly manipulate **widgets**; instead, you write **components** that return **elements**. The framework calls your components, gets a tree of elements, and builds a matching widget tree for you. When some event changes the application state, the framework calls your components again, gets a new element tree, and edits the widget tree accordingly.

As such, the root of your Panoramix application will usually look like:

```no_run
// main.rs

use panoramix::elements::{Button, ButtonClick};
use panoramix::{component, CompCtx, Element, NoEvent, RootHandler};

#[derive(Debug, Default, Clone, PartialEq)]
struct ApplicationState {
    // ...
}

#[component]
fn MyRootComponent(ctx: &CompCtx, _props: ()) -> impl Element<NoEvent, ApplicationState> {
    // ...
    # panoramix::elements::EmptyElement::new()
}

fn main() -> Result<(), panoramix::PlatformError> {
    let initial_state = ApplicationState {
        // ...
    };

    RootHandler::new(MyRootComponent::new(()))
        .with_initial_state(initial_state)
        .with_tracing(true)
        .launch()
}
```

For information on how to write a component, see [this document on Github](https://github.com/PoignardAzur/panoramix/blob/main/misc_docs/writing_a_component.md).
