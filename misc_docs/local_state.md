# Local state

This is part 3 of a 3-parts tutorial:

- [Writing a component](./writing_a_component.md)
- [Event handling](./event_handling.md)
- **Local state**

The third concept we need to write basic applications in Panoramix is local state.

Imagine we want to make a Counter. This component will have a `+` and a `-` button, as well as a label displaying the counter value. It might look like:

```rust
#[component]
fn Counter(_ctx: &CompCtx, _props: ()) -> impl Element {
    let current_count = 0;

    Row!(
        Label::new(format!("Count: {}", current_count)),
        Button::new("+")
            .on_click(|_, _| /* ... */),
        Button::new("-")
            .on_click(|_, _| /* ... */),
    )
}
```

What should our `on_click` events do?

They can't mutate `current_count`, because the value will be out of scope by the time a click is processed. We could pass `current_count` as a props, but it would have the same problem.

This is where local state comes in.

To add local state to our component, we add a second generic parameter to our `impl Element`:

```rust
use panoramix::NoEvent;

#[component]
fn Counter(ctx: &CompCtx, _props: ()) -> impl Element<NoEvent, i32> {
    // ...
}
```

(`NoEvent` is a bottom type that indicates events will never be thrown; it's equivalent to `!`)

The signature `-> impl Element<NoEvent, i32>` means "We return a component with an `i32` local state". For every instance of `Counter` our program creates, Panoramix will associate a persistent state of type `i32` with it, until the instance is destroyed.

TODO - mention ElementList

To read from our state, we call `CompCtx::use_local_state`:

```rust
#[component]
fn Counter(ctx: &CompCtx, _props: ()) -> impl Element<NoEvent, i32> {
    let current_count = ctx.use_local_state::<i32>();;

    Row!(
        Label::new(format!("Count: {}", current_count)),
        Button::new("+")
            .on_click(|_, _| /* ... */),
        Button::new("-")
            .on_click(|_, _| /* ... */),
    )
}
```

To *write* to our local state, we use the `on_click` callback. All event callbacks in Panoramix take a mutable reference to local state as their first parameter:

```rust
#[component]
fn Counter(ctx: &CompCtx, _props: ()) -> impl Element<NoEvent, i32> {
    let current_count = ctx.use_local_state::<i32>();;

    Row!(
        Label::new(format!("Count: {}", current_count)),
        Button::new("+")
            .on_click(|new_count: &mut i32, _event| *new_count += 1),
        Button::new("-")
            .on_click(|new_count: &mut i32, _event| *new_count -= 1),
    )
}
```

Note that, in this example, while `current_count` and `new_count` refer to the same value conceptually, their references are completely disjoint. By the time Panoramix calls the `on_click` callback, `current_count` and every other local we could have defined in our component have gone out of scope, which is why `on_click` provides a local state argument we can mutate.


## Local state and type inference

TODO

## Root state

TODO


## Conclusion

This should be enough for you to understand the basics of Panoramix.

If you need more context to intuit how the concepts fit together, look at the `examples/` folder. The concepts defined in this tutorial cover everything in them.

If you still have questions, you should visit [Druid's Zulip chat](https://xi.zulipchat.com/), in the `#panoramix` channel. If you think this tutorial is missing something, feel free to create a Github issue.
