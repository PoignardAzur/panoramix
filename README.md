# Capitaine

Capitaine is an experimental GUI framework for the Rust programming language.

The framework is **data-driven and declarative**, drawing some inspiration from [React](https://github.com/facebook/react), and implemented on top of the [Druid](https://github.com/linebender/druid) toolkit.

It aims to use **simple, idiomatic Rust**: Capitaine doesn't use unsafe code, cells, mutexes, or DSL macros.


## Getting started

Here is our "hello world" example:

```rust
use capitaine::elements::{Button, ButtonPressed, Label};
use capitaine::{make_row, ElementTree, ElementTreeExt, NoEvent, RootHandler};

#[derive(Debug, Default, Clone, PartialEq)]
struct HelloBoxState {
    count: i32,
}

fn hello_box(state: &HelloBoxState, _props: ()) -> impl ElementTree<HelloBoxState, NoEvent> {
    make_row!(
        Button::new("Say hello").on::<ButtonPressed, _>(|state: &mut HelloBoxState, _| {
            println!("Hello world - {}", state.count);
            state.count += 1;
        }),
        Label::new(format!("Hello count: {}", state.count)),
    )
}

fn main() -> Result<(), druid::PlatformError> {
    let state = HelloBoxState { count: 0 };

    RootHandler::new(&hello_box, state)
        .with_tracing(true)
        .launch()
}
```

See the documentation for details - "Writing your first component". **TODO**


## Contributing

Issues and PRs are welcome.

PRs that add "basic block" widgets in particular will be appreciated.

See [CONTRIBUTING.md] for the rules to follow when making a PR. Issues don't have a mandatory format, but if you submit one, please follow common-sense rules: be polite, be professional, have a plan to kill everyone you meet, and if you have a problem, please include detailed steps to reproduce.

See [ARCHITECTURE.md] for a high-level presentation of the project code.

To ask questions and discuss development work, go to [Druid's Zulip chat](https://xi.zulipchat.com/), in the `#capitaine` channel.


## Usage

In your `Cargo.toml`, add the following:

```toml
capitaine = "0.1.0"
```

If you want the bleeding-edge version, add the following instead:

```toml
capitaine = { git = "https://github.com/PoignardAzur/capitaine.git" }
```

**Note for Linux users:** Capitaine is built on top of Druid, using the GTK backend, which requires a dev version of GTK3. See [GTK installation page](https://www.gtk.org/docs/installations/linux/#installing-gtk-from-packages) for install info.

Eg on Ubuntu-based distributions, you should run `sudo apt install libgtk-3-dev`.


## Authors

This crate has been almost entirely written by Olivier FAURE. Any other contributor will be added to [AUTHORS.md].

This project has been possible thanks to the extremely clean and approchable work of Raph Levien and Colin Rofls, as well as some mentoring on their part, and general interaction with the Druid community. In particular, Capitaine is inspired from [Crochet](https://github.com/raphlinus/crochet/), hence the name.


## Roadmap

- [X] Implement the main logic.
 - [X] Virtual DOM constrution and reconciliation.
 - [X] Flex logic.
 - [X] Event logic.
- [] Write documentation.
 - [] Readme.
 - [] Top-level doc page.
 - [] Writing a component.
 - [] All symbols (types, traits, functions, etc).
- [] Write unit tests.
 - [X] Basic constructors.
 - [X] compute_diff algorithm.
 - [] Event handling
 - [] Widget mutation.
- [] Add TextBox element.
- [] Add testing backend to druid for unit tests.
- [] Add integration tests based on visual snapshots.
- [] Write some benchmarks.
- [] Add dynamically-typed BoxDynElement (probably using `std::Any`).
- [] Add immutable ConstElement.
- [] Make the API more idiot-proof (eg look at error messages when a component is written incorrectly).
- [] Add even more tracing/logging.
- [] Write `#[derive(Event)]` macro.
- [] Write macro attribute for type-checking components.
- [] Refactor event system internals (keep API as-is).
- [] Add AnyEvent type.
- [] Add "free-form" mode, that can be integrated into an external main loop (eg for game engines).
- [] Implement screenshot generator for examples.
- [] Implement accessibility features.
- [] Write devtools, similar to firefox and chrome's.

Also, not a specific item but something that needs to be done continuously:

- Add new basic elements and useful components.
