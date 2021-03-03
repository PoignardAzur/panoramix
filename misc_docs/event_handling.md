# Event handling

This is part 2 of a 3-parts tutorial:

- [Writing a component](./writing_a_component.md)
- **Event handling**
- [Local state](./local_state.md)

The components we've written so far are static. They draw things on-screen, but they don't process any user interaction.

Let's say that we want each of our "Hello, XXX" labels to have a button to say "hello" back. For now, we'll cheat and use `println!`, though we'll see later how to affect application state too.

First, we need to add a button to our component:

```rust
use panoramix::elements::Button;

#[component]
fn HelloText(_ctx: &CompCtx, props: String) -> impl Element {
    Row!(
        Label::new(format!("Hello, {}", props)),
        Button::new("Say hello"),
    )
}
```

Clicking that button doesn't actually do anything, though.

To hook it up with an event, we use the `Button::on_click` method:


```rust
#[component]
fn HelloText(_ctx: &CompCtx, props: String) -> impl Element {
    Row!(
        Label::new(format!("Hello, {}", props)),
        Button::new("Say hello")
            .on_click(|_, _| println!("{} says hello", props)),
    )
}
```

We might notice a few things about on_click:

- It takes a callback as a parameter. In this particular case the callback arguments don't matter, so we elide them.
- It's a builder method. It takes a Button by value, and returns a new component by value. Generally speaking, every element defined in Panoramix only has builder methods.


## Bubbling up events

Imagine that, for the sake of this exercise, we want to call println from `HelloEveryone` instead of `HelloText`. Maybe we only want Alice and Carol to be able to say hello. To do that, we need some way for HelloBox to "transmit" the events it gets from its button.

First, we modify HelloText's signature to emit an event:

```rust
use panoramix::elements::ButtonClick;

#[component]
fn HelloText(_ctx: &CompCtx, props: String) -> impl Element<ButtonClick> {
    // ...
}
```

This signature means we are writing a component that might raise a `ButtonClick` event at some point.

New, we use the trait `ElementExt`, which defines several builder methods for all elements. The method we need is `ElementExt::bubble_up`:


```rust
use panoramix::ElementExt;

#[component]
fn HelloText(_ctx: &CompCtx, props: String) -> impl Element<ButtonClick> {
    Row!(
        Label::new(format!("Hello, {}", props)),
        Button::new("Say hello")
            .bubble_up::<ButtonClick>(),
    )
}
```

This essentially tells our component "I want you to raise all elements of type `ButtonClick` from this button as if you were emitting them directly".

We can now catch ButtonClick events from the parent component HelloEveryone, using `ElementExt::on`:

```rust
#[component]
fn HelloEveryone(_ctx: &CompCtx, _props: ()) -> impl Element {
    // Bob and Damian don't get to say hello
    Column!(
        HelloBox::new("Alice".to_string())
            .on::<ButtonClick>(|_, _| println!("Alice says hello")),
        HelloBox::new("Bob".to_string()),
        HelloBox::new("Carol".to_string())
            .on::<ButtonClick>(|_, _| println!("Carol says hello")),
        HelloBox::new("Damian".to_string()),
    )
}
```

`ElementExt::on` is a more general form of `Button::on_click`; it essentially says "execute the callback if this element emits an event of type T".


## Conclusion

Our complete code looks like:

```rust
use panoramix::elements::Button;
use panoramix::elements::ButtonClick;
use panoramix::ElementExt;

use panoramix::{component, CompCtx, Element};
use panoramix::elements::Label;
use panoramix::RootHandler;
use panoramix::Column;

#[component]
fn HelloText(_ctx: &CompCtx, props: String) -> impl Element<ButtonClick> {
    Row!(
        Label::new(format!("Hello, {}", props)),
        Button::new("Say hello")
            .bubble_up::<ButtonClick>(),
    )
}

#[component]
fn HelloEveryone(_ctx: &CompCtx, _props: ()) -> impl Element {
    // Bob and Damian don't get to say hello
    Column!(
        HelloBox::new("Alice".to_string())
            .on::<ButtonClick>(|_, _| println!("Alice says hello")),
        HelloBox::new("Bob".to_string()),
        HelloBox::new("Carol".to_string())
            .on::<ButtonClick>(|_, _| println!("Carol says hello")),
        HelloBox::new("Damian".to_string()),
    )
}

fn main() -> Result<(), druid::PlatformError> {
    RootHandler::new(HelloEveryone::new(()))
        .launch()
}
```

So far we can react to user inputs in a limited way, but we can't actually use it to change application state. In [the next part](./local_state.md), we'll see how to represent application state so that events can modify it.
