# Writing a component

Components in Capitaine are plain old functions, that follow a specific format.

While Capitaine performs some non-intuitive logic in the background, component themselves are very much non-magical. The function you write is the function than Capitaine execute, with no hidden codegen or side-effects.

Where there *is* some background logic is in the arguments passed to your components, and how Capitaine processes the return value.

## Our first component

Let's say we want to start with something simple: a component that takes a name, and displays a label that says `Hello, $YOUR_NAME`.

The basic declaration will look like this:

```rust
fn hello_text(state: &StateType, props: PropsType) -> impl ElementTree<StateType, EventType> {
  todo!()
}
```

For now, we're not using local state, so `StateType` can be `()`, and we're not raising events, so `EventType` can be `capitaine::NoEvent` (will eventually be `!`).

We want to pass a string, so `PropsType` can be `&str`.

```rust
fn hello_text(state: &(), props: &str) -> impl ElementTree<(), NoEvent> {
  todo!()
}
```

Finally, we want to return a label that says hello:

```rust
fn hello_text(_state: &(), name: &str) -> impl ElementTree<(), NoEvent> {
  Label::new(format!("Hello, {}", props))
}
```

If we want to test our component in a program, we can pass it to `capitaine::RootHandler` from our main function:

```rust
fn main() -> Result<(), druid::PlatformError> {
    RootHandler::new(&hello_text, "John Doe")
        .launch()
}
```

TODO - screenshot

## Composing components

*Components* are called that way, because they're the unit of *composition* in our GUI.

For instance, let's say we want our GUI to say hello to multiple people at the same time. Maybe we want to say hello to Alice, Bob, Carol and Damian. We could copy-paste the above code multiple times:

```rust
fn hello_text(_state: &(), _props: ()) -> impl ElementTree<(), NoEvent> {
  make_group!(
    Label::new(format!("Hello, Alice", props)),
    Label::new(format!("Hello, Bob", props)),
    Label::new(format!("Hello, Carol", props)),
    Label::new(format!("Hello, Damian", props)),
  )
}
```

(`make_group!()` is a macro similar to `vec![]`, that takes a tuple of elements of arbitrary types and returns an element that contains them all; the value returned by `make_group` always implements `ElementTree`)

However, this is a very brute-force approach; we'd rather reuse code. (TODO) What we do instead is:

```rust
fn hello_text(_state: &(), name: &str) -> impl ElementTree<(), NoEvent> {
  Label::new(format!("Hello, {}", props))
}

fn hello_everyone(_state: &(), _props: ()) -> impl ElementTree<(), NoEvent> {
  make_group!(
    ComponentCaller::prepare(hello_text, "Alice"),
    ComponentCaller::prepare(hello_text, "Bob"),
    ComponentCaller::prepare(hello_text, "Carol"),
    ComponentCaller::prepare(hello_text, "Damian"),
  )
}
```

TODO - explain rationale

## Event handling

The components we've written so far are static. They don't have state, they don't process any user interaction.

Let's say that we want each of our "Hello, XXX" labels to have a button to say "hello" back. Moreover, we want our label to count the number of time the button has been pressed.

To do that, we change the `State` type of our component to an integer, that can hold the number of times the button was pressed:

```rust
fn hello_text(state: &u32, name: &str) -> impl ElementTree<u32, NoEvent> {
  // ...
}
```

In the component body, we add a parameter to the format macro, and we add a button:

```rust
fn hello_text(state: &u32, name: &str) -> impl ElementTree<u32, NoEvent> {
  make_group!(
    Label::new(format!("Hello, {} - ", props, state)),
    Button::new("Say hello")
    // ...
  )
}
```

While we've added a button, so far that button doesn't actually do anything.

To add a callback to our button, we use the trait `WidgetExt`, which has several callback-based methods. We use `WidgetExt::on`, and pass it a callback which takes both a mutable reference to the local state, and the value of the triggering event (here, a button press):

```rust
fn hello_text(state: &u32, name: &str) -> impl ElementTree<u32, NoEvent> {
  make_group!(
    Label::new(format!("Hello, {} - ", props, state)),
    Button::new("Say hello").on::<ButtonPressed, _>(|state: &mut u32, _event: ButtonPressed| {
      state += 1;
    }),
  )
}
```

Several things of note here:

- The callback takes a mutable reference on the `u32` state; which that state is conceptually the same as the one passed to hello_text, and mutating the first will affect the second, the *references* are completely different, and have disjoint lifetimes.
- The `on` method uses the builder pattern: it takes an own `impl ElementTree` as an input, and returns another `impl ElementTree` as an output.

TODO - note about not-having-magic and Debug

TODO - note about template type resolution

TODO - note about events and local state
