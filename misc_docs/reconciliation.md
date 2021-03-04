# Reconciliation in Panoramix

Disclaimer: This is somewhat of a first draft, and it's probably going to be unstructured, even compared to the rest of the documentation.

## About the virtual DOM

The architecture of Panoramix is centered around a concept called "Virtual DOM" (though I don't know if it has other names in other frameworks). The name comes from browser-based GUI frameworks like React and Vue.js; in the browser, the DOM (Document Object Model) is the abstraction to manipulate the content of a webpage dynamically. It's a powerful abstraction, but it stores a lot of data and it's expensive to manipulate.

To address this, JS frameworks use what's called a Virtual DOM. The idea is that after every event, user-made functions return a tree of lightweight descriptions of GUI elements. These descriptions are cheap to generate and cheap to manipulate.

Once an entire tree is generated, the framework compares it with the previous version of that tree, detects which bits change, and propagates these changes to the actual DOM.

This process is called **reconciliation**.


## How Panoramix does it

While Panoramix is built on top of druid, not a DOM, the general process is the same.

Modifying the widget tree is handled by two functions, `VirtualDom::init_tree` and `VirtualDom::reconcile`:

- `init_tree` takes an element and creates a druid Widget from it.
- `reconcile` takes and old and a current version of an element, and applies the differences to a given druid Widget.

For instance, `elements::Label::init_tree` will return a `druid::widget::Label`; while `elements::Label::reconcile` will take a mutable reference to a `druid::widget::Label` and modify it (for instance, changing its text) to reflect the label's new value.

Both of those are recursive methods: for any container element (such as `ElementList`, or the elements returned by builder methods like `on_click`), calling `init_tree` and `reconcile` will call the respective methods on its children.


## Widget permanence

Widget permanence is the notion that each widget, once created, has a unique "identity" and that if a function returns something like:

```rust
Column!(
    Label::new("Hello"),
    Label::new("Bar"),
    TextBox::new(content),
)
```

then the framework should "know" that these elements are still the same after an update and keep local state information, selection information, where the cursor is in the text box, etc.

Most elements don't need to worry about widget permanence, because it's statically defined: unlike in React, where a function might return a `<div>` on one frame and a `<ol>` on the next, in Panoramix any function that is defined to return a label will always return a label. Similarly, containers like `Row!` and `Column!` are also statically typed.

(theoretically, this has the power to skip a lot of the computations that virtual DOM frameworks have to go through; in practice, we're definitely not at the stage where we can compete on performance yet)

For now, the one container that has to think about widget permanence is ElementList. It handles it by asking that every element be provided with a unique key.

For instance, if your element list before update looks like:

``` text
[
    ("foo-1", MyComponent("foo")),
    ("foo-2", MyComponent("foo")),
    ("bar-3", MyComponent("bar")),
];
```

 and your element list post-update looks like:

``` text
[
    ("foo-1", MyComponent("foo")),
    ("bar-3", MyComponent("bar")),
];
```

ElementTree will figure out that the element at `foo-2` has been removed, and will remove it from the underlying widget tree, as well as perform any necessary cleanup.

Option and Either also handle widget identity and cleanup, but it's a lot less complicated since the only possible states are "widget is there" or "widget isn't there" with no possible confusion on order or anything.
