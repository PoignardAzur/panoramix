Architecture
============

This document describes the high-level architecture of Panoramix.

* *Note:* This document had its last update around March 14, 2021; if this message is still here by, let's say, Sept 15 2021, feel free to open an issue or ping me.

First off, let's make things clear:

- This is an **internal** documentation, not intended for end users. If you want to use Panoramix as a dependency, look at the docs.rs doc; **TODO - add link**
- This is an **informal** documentation, loosely following the guidelines described in [matklad's post](https://matklad.github.io/2021/02/06/ARCHITECTURE.md.html). The idea is to make this easy to update every few months, not exhaustively document every piece of code.
- This documentation assumes you have some familiarity with Panoramix and have at least compiled a few examples.
- This documentation is a little sparse and vague, by design. I strongly recommend reading it side by side with an open text editor, so you can read the code at the same time.

That being said, if you're looking to contribute to Panoramix and you don't know where to begin, you're in the right place!

- [Design principles](design-principles)
  - [Panoramix is functional and declarative](panoramix-is-functional-and-declarative)
  - [Panoramix is not clever](panoramix-is-not-clever)
  - [Panoramix uses static types](panoramix-uses-static-types)
- [Project structure](#project-structure)
  - [`elements`](#elements)
  - [`widgets`](#widgets)
  - [`root_handlerrs`](#root_handlerrs)


## Design principles

Before we talk about code structure, here are the general design principles of panoramix.

### Panoramix is functional and declarative

For those familiar with React, this means we want [function components, not class components](https://reactjs.org/docs/components-and-props.html). Writing a piece of GUI in Panoramix looks like this pseudocode:

```rust
fn my_component(input, other_input) -> OutputToDisplay {
    return ElementGroup(
        Label("some text"),
        Button("button text")
            .on_press(do_stuff),
        OtherComponent(props),
    );
}
```

As opposed to this:

```rust
struct MyComponent{
    state: SomeState,
    state2: SomeMoreState,
}

impl ComponentLifecycle {
    fn create() { ... }
    fn update() { ... }
    fn display() { ... }
}
```

This is a central choice of the framework. I believe it makes writing component easier, and more intuitive.

The abstraction that backs this choice is called a "Virtual DOM". See [this document](./reconciliation) for details.


### Panoramix is not clever

The common wisdom is that Rust isn't a great language for writing a GUI framework. GUIs require lots of complicated patterns to mutate data in inconvenient ways. Widgets in traditional frameworks usually hold a reference to their parent, and can run callbacks at arbitrary points in the code, that can mutate data that other widgets access, also at arbitrary points in the code.

To make sense of that logic in Rust, one might be tempted to use some of Rust's more arcane features: unsafe code, RefCells, Arcs, mutexes, and so on. To be able to reason about widget hierarchy, one might be tempted to define them with DSL macros, and this is indeed what other Rust GUI frameworks do.

Not here. We do everything the hard way. If you can't get a mutable reference to some data in safe code, you can't mutate that data and that's final.

The good news is, making a GUI that works in Rust without using these patterns is surprisingly doable. You should read the `VirtualDom` trait and its implementations for details, but the general pattern is:

- The framework holds a tree of **states**, which are application-defined PODs.
- The application-defined components each borrows a given **state** (as well as **props** passed from other components), and generate a GUI from them, as well as **event callbacks**.
- The GUI is shown to the user.
- Depending on user-interaction, some **event callbacks** are called with a mutable reference to their component **state**.
- If the **callbacks** have changed the **state** value, we call the **components** again to regenerate the GUI, and so on.

<!-- (TODO - rewrite this) -->

This is possible because once we get to event processing, the application-defined components are no longer borrowing their state.

<!-- TODO - See "twin visitors" -->


### Panoramix uses static types

This is less important than the above two, but still part of the original design.

Many virtual DOM frameworks (especially in JS) will have dynamically-typed representation of their virtual DOM. So the output of a component might look like:

```
VirtualDomNode("list")
- VirtualDomNode("button", data = ...)
- VirtualDomNode("label", data = "sometext")
- VirtualDomNode("row")
 - VirtualDomNode("label")
 - VirtualDomNode("label")
```

Conceptually, the output of a Panoramix component is more like:

```
ListNode
- ButtonNode(data = ...)
- LabelNode(data = "sometext")
- RowNode
 - LabelNode
 - LabelNode
```

Being statically-typed means that Panoramix gets to skip some redundant checks during reconciliation (eg if it previously had a "Label" node, it doesn't need to check that the new node is still a label, though it still needs to compare their text).


## Project structure

The important traits are:

- **Element**: What every user-facing function returns.
- **VirtualDom**: Same thing, but one step removed. VirtualDom items are built from elements and their types define methods for building the widget tree, reconciliation and event handling. This is where most of the magic happens.
- **WidgetSequence**: wrapper for iterators of FlexWidgets. Most elements (eg Label, Button, TextBox, Flex containers) are sequences of a single FlexWidget. Non-Flex containers (Tuple, ElementList, Option, Either) are sequences of 0-to-many widgets. (for instance Option is 0-to-1)
- **FlexWidget**: wrapper for a `druid::WidgetPod` plus some flex data. Object-safe.

Essentially, every element the user can instantiate has to bind to all four of these traits. You can think of Element as the front-end and druid as the backend.

## `elements/`

Where `Element` and `VirtualDom` implementations are defined.

## `widgets/`

Where `WidgetSequence` and `FlexWidget` implementations are defined.

## `root_handler.rs`

Defines the functions that users call in their `main()` function to start the app.
