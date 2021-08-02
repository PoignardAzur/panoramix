# Local state

This is part 3 of a 3-parts tutorial:

- [Writing a component](./writing_a_component.md)
- [Event handling](./event_handling.md)
- **Local state**

The third concept we need to write basic applications in Panoramix is local state.

Imagine we want to make a Counter. This component will have a `+` and a `-` button, as well as a label displaying the counter value. It might look like:

```rust
#[component]
fn Counter(ctx: &CompCtx, props: ()) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<NoEvent, ()>();
    let current_count = 0;

    Row!(
        Label::new(format!("Count: {}", current_count)),
        Button::new("+")
            .on_click(md, |_, _| /* ... */),
        Button::new("-")
            .on_click(md, |_, _| /* ... */),
    )
}
```

What should our `on_click` events do?

They can't mutate `current_count`, because the value will be out of scope by the time a click is processed. We could pass `current_count` as a props, but it still wouldn't live long enough.

This is where local state comes in.

To add local state to our component, modify the second type parameter passed to `use_metadata`:

```rust
use panoramix::NoEvent;

#[component]
fn Counter(ctx: &CompCtx, props: ()) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<NoEvent, i32>();
    // ...
}
```

Calling `ctx.use_metadata::<NoEvent, i32>()` means "We return a component with an `i32` local state". For every instance of `Counter` our program creates, Panoramix will associate a persistent state of type `i32` with it, until the instance is destroyed.

*TODO - mention ElementList*

To read from our state, we call `CompCtx::get_local_state`:

```rust
#[component]
fn Counter(ctx: &CompCtx, props: ()) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<NoEvent, i32>();
    let current_count = ctx.get_local_state(md);

    ComponentOutput::new(
        md,
        Row!(
            Label::new(format!("Count: {}", current_count)),
            Button::new("+")
                .on_click(md, |_, _| /* ... */),
            Button::new("-")
                .on_click(md, |_, _| /* ... */),
        ),
    )
}
```

To *write* to our local state, we use the `on_click` callback. All event callbacks in Panoramix take a mutable reference to local state as their first parameter:

```rust
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

Note that, in this example, while `current_count` and `new_count` refer to the same value conceptually, their references are completely disjoint. By the time Panoramix calls the `on_click` callback, `current_count` and every other local we could have defined in our component have gone out of scope, which is why `on_click` provides a local state argument we can mutate.


## Root state and default value

TODO


## Conclusion

This should be enough for you to understand the basics of Panoramix.

If you need more context to intuit how the concepts fit together, look at the `examples/` folder. The concepts defined in this tutorial cover everything in them.

If you still have questions, you should visit [Druid's Zulip chat](https://xi.zulipchat.com/), in the `#panoramix` channel. If you think this tutorial is missing something, feel free to create a Github issue.
