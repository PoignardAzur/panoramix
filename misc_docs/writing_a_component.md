# Writing a component

This is part 1 of a 3-parts tutorial:

- **Writing a component**
- [Event handling](./event_handling.md)
- [Local state](./local_state.md)

Components in Panoramix are plain old functions, that follow a specific format.

While Panoramix performs some non-intuitive logic in the background, component themselves are very much non-magical. The function you write is the function than Panoramix executes, with no hidden codegen or side-effects.

Where there *is* some background logic is in the arguments passed to your components, and how Panoramix processes the return value.

## Our first component

Let's say we want to start with something simple: a component that takes no parameter, and displays a label that says `Hello world:`.

The basic declaration will look like this:

```rust
use panoramix::{component, CompCtx, Element, NoEvent};

#[component]
fn HelloText(ctx: &CompCtx, props: ()) -> impl Element<Event = NoEvent> {
    todo!()
}
```

(For now, ignore the `ctx`, `props` and `Event = NoEvent` parts; for an example this simple, they can be considered boilerplate.)

The `#[component]` attribute mostly just reads the function prototype, and generates a `struct HelloText` type from it with a few inherent methods and trait implementations. We'll come back to that later.

Our component prototype is parameterized with a props type and an event type. A component's props are the arguments its uses to generate its GUI; the event type describes the events it can emit from user interaction. In our case, we take no parameter and emit no events, so these types are respectively `()` and `NoEvent`.

Finally, we want to return a label that says hello:

```rust
use panoramix::elements::Label;

#[component]
fn HelloText(ctx: &CompCtx, props: ()) -> impl Element<Event = NoEvent> {
    Label::new("Hello world!")
}
```

If we want to test our component in a program, we have to pass it to `panoramix::RootHandler` from our main function.

The `#[component]` attribute generates a `HelloText` unit type, that implements the trait `Component`. `RootHandler` has a constructor that expects a `Component`, so we can pass it our `HelloText`:

```rust
use panoramix::RootHandler;

fn main() -> Result<(), panoramix::PlatformError> {
    RootHandler::new(HelloText)
        .launch()
}
```

*TODO - screenshot*


## Composing components

*Components* are called that way, because they're the unit of *composition* in a declarative GUI.

For instance, let's say we want our GUI to say hello to specific people. Maybe we want to say hello to Alice, Bob, Carol and Damian. We could copy-paste the above code multiple times:

```rust
use panoramix::Column;

#[component]
fn HelloEveryone(ctx: &CompCtx, props: ()) -> impl Element<Event = NoEvent> {
    Column!(
        Label::new("Hello Alice!"),
        Label::new("Hello Bob!"),
        Label::new("Hello Carol!"),
        Label::new("Hello Damian!"),
    )
}
```

(`Column!()` is a macro similar to `vec![]`, that takes a tuple of elements of arbitrary types and returns an element that contains them all; the value returned by `Column` always implements `Element`)

But this is obviously a pattern where we want to compose code, not copy-paste it. This is where props become useful: because we have defined a component parameterized on a name, we can just build that component multiple times with different values.

First, we change the component to take a String prop:

```rust
#[component]
fn HelloText(ctx: &CompCtx, props: String) -> impl Element<Event = NoEvent> {
    Label::new(format!("Hello {}!", props))
}
```

The `#[component]` attribute generates a `HelloText` type, with a `new()` method that takes a string:

```rust
HelloText::new(String::from("Foobar"))
```

Our complete code looks like:

```rust
use panoramix::{component, CompCtx, Element};
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
    RootHandler::new(HelloEveryone)
        .launch()
}
```

We're able to combine these different types seamlessly because `Column!` takes arbitrary elements as parameters, and `HelloText::new` returns an element.


## About magic

A high-level goal of Panoramix is to avoid magical DSLs where the code you write isn't the code that gets executed.

As part of this, all elements we have built (with `Label::new` and `Column!` and `HelloText::new`) are simple PODs, with no hidden cells or `Arc<Mutex>`. More over, all elements (including components) are required to implement `Debug`, which means you can print the elements you're building at any point:

```rust
#[component]
fn HelloEveryone(_ctx: &CompCtx, _props: ()) -> impl Element {
    let first_label = Label::new(format!("Hello, Alice"));
    println!("first_label: {:#?}", first_label);

    let column = Column!(
        first_label,
        HelloText::new("Bob".to_string()),
        HelloText::new("Carol".to_string()),
        HelloText::new("Damian".to_string()),
    );

    println!("===");
    println!("column: {:#?}", column);
    column
}
```

This should display something like:

```text
first_label: Label {
    text: "Hello, Alice",
    flex: FlexParams {
        flex: 1.0,
        alignment: None,
    },
},
===
column: Flex {
    axis: Vertical,
    child: ElementTuple_4(
        Label {
            text: "Hello, Alice",
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
        },
        HelloText {
            props: "Hello, Bob",
        },
        HelloText {
            props: "Hello, Carol",
        },
        HelloText {
            props: "Hello, Damian",
        },
    ),
    // ...
}
```

You might notice that, at the time we return them, instances of `HelloText` just contain props. That's because the actual `HelloText` function is called after `HelloEveryone` has returned.

## ElementList

TODO


## Conclusion

Our complete code looks like:

```rust
use panoramix::{component, CompCtx, Element};
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
    RootHandler::new(HelloEveryone)
        .launch()
}
```

In [the next part](./event_handling.md), we will see how we can make our component react to user input.
