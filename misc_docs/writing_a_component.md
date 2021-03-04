# Writing a component

This is part 1 of a 3-parts tutorial:

- **Writing a component**
- [Event handling](./event_handling.md)
- [Local state](./local_state.md)

Components in Panoramix are plain old functions, that follow a specific format.

While Panoramix performs some non-intuitive logic in the background, component themselves are very much non-magical. The function you write is the function than Panoramix execute, with no hidden codegen or side-effects.

Where there *is* some background logic is in the arguments passed to your components, and how Panoramix processes the return value.

## Our first component

Let's say we want to start with something simple: a component that takes a name, and displays a label that says `Hello, $YOUR_NAME`.

The basic declaration will look like this:

```rust
use panoramix::{component, CompCtx, Element};

#[component]
fn HelloText(_ctx: &CompCtx, props: __PROPS_TYPE__) -> impl Element {
    todo!()
}
```

(More complicated examples will add generic parameters to `Element`, but for now we're using their default values.)

The `#[component]` attribute mostly just reads the function prototype, and generates a `struct HelloText` type from it with a few inherent methods. We'll come back to that later.

A component's props are the arguments its uses to generate its GUI. In this case, we want to take a name, so we can replace `__PROPS_TYPE__` with `String`:

```rust
#[component]
fn HelloText(_ctx: &CompCtx, props: String) -> impl Element {
    todo!()
}
```

Finally, we want to return a label that says hello:

```rust
use panoramix::elements::Label;

#[component]
fn HelloText(_ctx: &CompCtx, props: String) -> impl Element {
    Label::new(format!("Hello, {}", props))
}
```

If we want to test our component in a program, we have to pass it to `panoramix::RootHandler` from our main function.

The `#[component]` attribute generates a `HelloText::new` constructor for our component, that takes its props as parameters and returns an element (see the attribute's documentatation for details).

We can use it to generate an instance of our component that our application will display:

```rust
use panoramix::RootHandler;

fn main() -> Result<(), panoramix::PlatformError> {
    RootHandler::new(HelloBox::new("World".to_string()))
        .launch()
}
```

TODO - screenshot


## Composing components

*Components* are called that way, because they're the unit of *composition* in a declarative GUI.

For instance, let's say we want our GUI to say hello to multiple people at the same time. Maybe we want to say hello to Alice, Bob, Carol and Damian. We could copy-paste the above code multiple times:

```rust
use panoramix::Column;

#[component]
fn HelloText(_ctx: &CompCtx, _props: ()) -> impl Element {
    Column!(
        Label::new(format!("Hello, Alice")),
        Label::new(format!("Hello, Bob")),
        Label::new(format!("Hello, Carol")),
        Label::new(format!("Hello, Damian")),
    )
}
```

(`Column!()` is a macro similar to `vec![]`, that takes a tuple of elements of arbitrary types and returns an element that contains them all; the value returned by `Column` always implements `Element`)

This is obviously the kind of pattern we want to abstract into function calls: we have a function parameter that changes (the name), and a function body that stays the same (the label). So what we do is build our component with different props each time, using the `HellowText::new` syntax we used earlier:

But this is obviously a pattern where we want to compose code, not copy-paste it. This is where props become useful: because we have defined a component parameterized on a name, we can just build that component multiple times with different values.

Our complete code looks like:

```rust
use panoramix::{component, CompCtx, Element};
use panoramix::elements::Label;
use panoramix::RootHandler;
use panoramix::Column;

#[component]
fn HelloText(_ctx: &CompCtx, props: String) -> impl Element {
    Label::new(format!("Hello, {}", props))
}

#[component]
fn HelloEveryone(_ctx: &CompCtx, _props: ()) -> impl Element {
    Column!(
        HelloBox::new("Alice".to_string()),
        HelloBox::new("Bob".to_string()),
        HelloBox::new("Carol".to_string()),
        HelloBox::new("Damian".to_string()),
    )
}

fn main() -> Result<(), panoramix::PlatformError> {
    RootHandler::new(HelloEveryone::new(()))
        .launch()
}
```

We managed to combine these different types seamlessly because `Column!` takes arbitrary elements as parameters, and `HelloBox::new` returns an element.


## About magic

A high-level goal of Panoramix is to avoid magical DSLs where the code you write isn't the code that gets executed.

As part of this, all elements we have built (with `Label::new` and `Column!` and `HelloText::new`) are simple PODs, with no hidden cells or `Arc<Mutex>`. More over, all elements (including components) are required to implement `Debug`, which means you can print the elements you're building at any point:

```rust
#[component]
fn HelloText(_ctx: &CompCtx, _props: ()) -> impl Element {
    let first_label = Label::new(format!("Hello, Alice"));
    println!("first_label: {:#?}", first_label);

    let column = Column!(
        first_label,
        HelloBox::new("Bob".to_string()),
        HelloBox::new("Carol".to_string()),
        HelloBox::new("Damian".to_string()),
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
        HelloBox {
            props: "Hello, Bob",
        },
        HelloBox {
            props: "Hello, Carol",
        },
        HelloBox {
            props: "Hello, Damian",
        },
    ),
    // ...
}
```

You might notice that, at the time we return them, instances of `HelloBox` just contain props. That's because the actual function is called a bit later on.

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
fn HelloText(_ctx: &CompCtx, props: String) -> impl Element {
    Label::new(format!("Hello, {}", props))
}

#[component]
fn HelloEveryone(_ctx: &CompCtx, _props: ()) -> impl Element {
    Column!(
        HelloBox::new("Alice".to_string()),
        HelloBox::new("Bob".to_string()),
        HelloBox::new("Carol".to_string()),
        HelloBox::new("Damian".to_string()),
    )
}

fn main() -> Result<(), panoramix::PlatformError> {
    RootHandler::new(HelloEveryone::new(()))
        .launch()
}
```

In [the next part](./event_handling.md), we will see how we can make our component react to user input.
