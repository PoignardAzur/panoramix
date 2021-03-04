# Panoramix

[![crates.io](https://meritbadge.herokuapp.com/panoramix)](https://crates.io/crates/panoramix)
[![docs.rs](https://docs.rs/panoramix/badge.svg)](https://docs.rs/panoramix/)
[![license](https://img.shields.io/github/license/PoignardAzur/panoramix)](https://github.com/linebender/druid/blob/master/LICENSE)
[![chat](https://img.shields.io/badge/zulip-join_chat-brightgreen.svg)](https://xi.zulipchat.com)

**Panoramix is an experimental GUI framework for the Rust programming language.**

The framework is **data-driven and declarative**, drawing some inspiration from [React](https://github.com/facebook/react), and implemented on top of the [Druid](https://github.com/linebender/druid) toolkit (hence the name).

It aims to use **simple, idiomatic Rust**: Panoramix doesn't use unsafe code, cells, mutexes, or DSL macros.


## Getting started

Here is our "hello world" example:

```rust
use panoramix::elements::{Button, Label};
use panoramix::{component, Column, CompCtx, Element, NoEvent, RootHandler};

#[component]
fn HelloBox(_ctx: &CompCtx, _props: ()) -> impl Element {
    Column!(
        Label::new("Hello world!"),
        Button::new("Say hello").on_click(|_, _| {
            println!("Hello world");
        })
    )
}

fn main() -> Result<(), druid::PlatformError> {
    RootHandler::new(HelloBox::new(()))
        .with_tracing(true)
        .launch()
}
```

See [the documentation](https://docs.rs/panoramix/) for details, and the [Writing a new component](misc_docs/writing_a_component.md) tutorial.


## Contributing

Issues and PRs are welcome.

PRs that add "basic block" widgets in particular will be appreciated.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the rules to follow when making a PR. Issues don't have a mandatory format, but if you submit one, please follow common-sense rules: be polite, be professional, have a plan to kill everyone you meet, and if you have a problem, please include detailed steps to reproduce.

See [ARCHITECTURE.md](ARCHITECTURE.md) for a high-level presentation of the project code.

To ask questions and discuss development work, go to [Druid's Zulip chat](https://xi.zulipchat.com/), in the `#panoramix` channel.


## Usage

In your `Cargo.toml`, add the following:

```toml
panoramix = "0.1.0"
```

If you want the bleeding-edge version, add the following instead:

```toml
panoramix = { git = "https://github.com/PoignardAzur/panoramix.git" }
```

**Note for Linux users:** Panoramix is built on top of Druid, using the GTK backend, which requires a dev version of GTK3. See [GTK installation page](https://www.gtk.org/docs/installations/linux/#installing-gtk-from-packages) for install info.

Eg on Ubuntu-based distributions, you should run `sudo apt install libgtk-3-dev`.


## Roadmap

The short term roadmap is:

- Write more tests, and try to make everything we use more testable (notably Druid).
- Write benchmarks.
- Improve the error story, and write helper macros.
- Try to streamline the common, simple tasks (eg writing a task list, writing a menu, etc) as much as possible.

On the longer timescale, there are two broad goals:

- Achieve feature parity with React.
- Use Panoramix to build a debugging framework, similar to chrome-devtools. TODO - Write blog posts about this.

See [ROADMAP.md](ROADMAP.md) for details.


## Authors

This crate has been almost entirely written by Olivier FAURE. Any other contributor will be added to [AUTHORS.md](AUTHORS.md).

This project has been possible thanks to the extremely clean and approchable work of Raph Levien and Colin Rofls, as well as some mentoring on their part, and general interaction with the Druid community. In particular, Panoramix is inspired from [Crochet](https://github.com/raphlinus/crochet/).
