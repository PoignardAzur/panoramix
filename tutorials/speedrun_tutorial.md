# Speedrun tutorial

This is a quick-and-dirty tutorial for people who already have experience with web frameworks. This tutorial assumes you already know the main concepts, and explains how Panoramix relates to them. If you want to more step-by-step introduction, read [the main tutorial](t_01_writing_a_component) instead.


## General concept

Panoramix is a Virtual-DOM-based framework inspired by React.

Whereas React and other Web frameworks often have a "class" syntax and a "function" syntax for components (and other Rust framework often mimick the class syntax with traits), Panoramix only offers a function syntax.

A Panoramix component therefore looks like this:

```rust
# use panoramix::{component, CompCtx, Element, NoEvent};
# use panoramix::elements::Label;
# use panoramix::RootHandler;
# use panoramix::Column;
#[component]
fn HelloText(ctx: &CompCtx, props: String) -> impl Element<Event = NoEvent> {
    Label::new(format!("Hello {}!", props))
}
```

Some details to note:

- Components are functions annotated with the `#[component]` attribute.
- Like in React, component names are UpperCamelCase. This is actually enforced with compiler warnings.
- All components return `impl Element`, where [Element](crate::Element) is the trait that defines the building blocks of components. Eg labels, buttons, rows and columns are elements.
- The first function argument is [CompCtx](crate::CompCtx), a context parameter (more on this in a bit).
- The second argument is the props.

The `#[component]` attribute then generates a unit `HelloText` type, which implements the `Component` trait, and has a convenience `new(props)` method that can be called from other components:

```rust
# use panoramix::elements::Label;
# type HelloText = Label;
HelloText::new(String::from("World"))
# ;
```

This creates a tree of composition whose root is passed to the application launcher:

```rust
use panoramix::{component, CompCtx, Element, NoEvent};
use panoramix::elements::Label;
use panoramix::RootHandler;
use panoramix::Column;

#[component]
fn HelloText(ctx: &CompCtx, props: String) -> impl Element<Event = NoEvent> {
    Label::new(format!("Hello {}!", props))
}

#[component]
fn HelloEveryone(ctx: &CompCtx, props: ()) -> impl Element<Event = NoEvent> {
    Column!(
        HelloText::new("Alice".to_string()),
        HelloText::new("Bob".to_string()),
        HelloText::new("Carol".to_string()),
        HelloText::new("Damian".to_string()),
    )
}

fn main() -> Result<(), panoramix::PlatformError> {
    # return Ok(());
    RootHandler::new(HelloEveryone)
        .launch()
}
```

The application runs the root component and gets a tree of elements, and builds a widget hierarchy from it. When the application state is changed (in the above example, never, since these components are stateless), the application calls the root component again and updates the widget hierarchy to propagate the changes, much like a JS virtual DOM framework.


## Local state

The above components were stateless. Below is an example of a stateful component, a counter with buttons to increment and decrement its value.

```rust
# use panoramix::{component, CompCtx, Element, NoEvent};
# use panoramix::elements::{Button, ComponentOutput, Label};
# use panoramix::Row;
#[component]
fn Counter(ctx: &CompCtx, props: ()) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<NoEvent, i32>();
    let current_count = ctx.get_local_state(md);

    ComponentOutput::new(
        md,
        Row!(
            Label::new(format!("Count: {}", current_count)),
            Button::new("+")
                .on_click(md, |new_count: &mut i32, _event| *new_count += 1),
            Button::new("-")
                .on_click(md, |new_count: &mut i32, _event| *new_count -= 1),
        ),
    )
}
```

Details of note:

- We call `ctx.use_metadata` to specificy metadata types for the component. `NoEvent, i32` means the component doesn't emit events, and has an `i32` local state. The function returns a metadata "token", that we need to pass to functions whose behavior depends on component metadata.
- We then call `ctx.get_local_state` with this token, which returns a shared reference to local state. This is similar in principle to React Hooks: local state is initialized to a default value at first, and can be changed by GUI events; for any given instance of a component, local state is persistent between calls to the component.
- We use the [`on_click`](crate::elements::Button::on_click) method to react to user events. **on_click** takes a metadata token, and a callback that gets a mutable reference to the current local state.
- Finally, we wrap the return value in a `ComponentOutput`. This is necessary to perform some behind-the-scenes type magic so that local state is properly managed. (It might be easy to forget that step, but if you do the component will panic with an error message immediately explaining what's missing)


## Component events

Panoramix uses an Elm-like event system.

Instead of taking callback parameters like in React, components declare what type of events they can emit, and parent components capture the events they're interested in.

So far all our components have been declared as `impl Element<Event = NoEvent>`, meaning they never emit events ([`NoEvent`](crate::NoEvent) is a bottom type).

We can declare a component that emits custom events like so:

```rust
# use panoramix::{component, CompCtx, Element, NoEvent};
# use panoramix::elements::{Label, Button, ComponentOutput};
# use panoramix::Row;
use panoramix::ElementExt;

#[derive(Debug, Clone, Default, PartialEq)]
struct MyEvent(i32);

#[component]
fn MyButton(ctx: &CompCtx, props: String) -> impl Element<Event = MyEvent> {
    let md = ctx.use_metadata::<MyEvent, ()>();
    ComponentOutput::new(
        md,
        Row!(
            Button::new("Click me!")
                .map_event(md, |_, _| Some(MyEvent(42))),
        ),
    )
}
```

Some notes:

- We now return `impl Element<Event = MyEvent>`; the type of event emitted is part of the function signature.
- We need to use ComponentOutput again. This is because ComponentOutput "transfers" the event type you specify into its associated type. In other words, if you do `let md = ctx.use_metadata::<Foobar, ...>();`, then `ComponentOutput::new(md, ...)` will be of type `impl Element<Event = Foobar>`.
- We import [`panoramix::ElementExt`](crate::ElementExt) and use its method [`map_event`](crate::ElementExt::map_event). This tells the component "transform the events of this button into the events I return".

`MyButton` can now emit events, that can be used from other components, eg:


```rust
# use panoramix::{component, CompCtx, Element, NoEvent};
# use panoramix::elements::{Button, ComponentOutput, Label};
# use panoramix::Row;
use panoramix::ElementExt;

#[derive(Debug, Clone, Default, PartialEq)]
struct MyEvent(i32);

#[component]
fn MyButton(ctx: &CompCtx, props: ()) -> impl Element<Event = MyEvent> {
    let md = ctx.use_metadata::<MyEvent, ()>();
    ComponentOutput::new(
        md,
        Row!(
            Button::new("Click me!")
                .map_event(md, |_, _| Some(MyEvent(42))),
        ),
    )
}

#[component]
fn Counter(ctx: &CompCtx, props: ()) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<NoEvent, i32>();
    let current_count = ctx.get_local_state(md);

    ComponentOutput::new(
        md,
        Row!(
            MyButton::new(())
                .on(md, |new_count: &mut i32, _event| *new_count += 1),
        ),
    )
}
```

(we use the method [`ElementExt::on`](crate::ElementExt::on), which is a more general version of `Button::on_click`)


## Static typing

Something you may have notice is that everything we've written so far was statically typed. There's no `Box<dyn Element>` or other type erasure. Instead, our components return `impl Element<...>`, and the compiler figures out what the types involved are.

This is good for performance (especially for reconciliation), but this is a little more restrictive than in a JS framework where any element can be substituted for any other.

In particular, this means the following:

```compile_fail
# use panoramix::{component, CompCtx, Element, NoEvent};
# use panoramix::elements::{Button, ComponentOutput, Label};
# use panoramix::Row;
#[component]
fn MyComponent(ctx: &CompCtx, props: bool) -> impl Element<Event = NoEvent> {
    if props {
      Button::new("Foo")
    } else {
      Label::new("Bar")
    }
}
```

will fail to compile, because [`Button`](crate::elements::Button) and [`Label`](crate::elements::Label) are two different types.

There are workarounds, though. In particular, both `Option<E>` and `Either<E1, E2>` (from the `either` crate) implement `Element`, and long as E/E1/E2 implement `Element`. That means the above component can be written as follows:

```rust
# use panoramix::{component, CompCtx, Element, NoEvent};
# use panoramix::elements::{Button, ComponentOutput, Label};
# use panoramix::Row;
use either::{Left, Right};

#[component]
fn MyComponent(ctx: &CompCtx, props: bool) -> impl Element<Event = NoEvent> {
    if props {
      Left(Button::new("Foo"))
    } else {
      Right(Label::new("Bar"))
    }
}
```


## Conclusion

This should be enough for you to understand the basics of Panoramix. There always the `examples/` folder, the documentation, the unit tests, etc, if you need more.

If you still have questions, you should visit [Druid's Zulip chat](https://xi.zulipchat.com/), in the `#panoramix` channel. If you think this tutorial is missing something (keeping in mind it's meant to be short), feel free to create a Github issue.
