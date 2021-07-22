# Roadmap

## Short term

- [X] Implement the main logic.
  - [X] Virtual DOM constrution and reconciliation.
  - [X] Flex logic.
  - [X] Event logic.
- [X] Write initial documentation.
  - [X] Readme.
  - [X] Top-level doc page.
  - [X] Writing a component.
  - [X] All symbols (types, traits, functions, etc).
- [ ] Write tests.
  - [X] Basic constructors.
  - [X] compute_diff algorithm.
  - [X] Event handling
    - [ ] Test event chaining
  - [X] Widget mutation.
  - [ ] Add integration test using all components, and snapshot-test the return values of every method.
- [X] Add TextBox element.
- [X] Add testing backend to druid for unit tests.
- [ ] Write some benchmarks.
- [X] Add dynamically-typed BoxDynElement (probably using `std::Any`).
- [ ] Add immutable ConstElement.
- [ ] Refactor ElementList to allow duplicate keys
- [ ] Fix component macro error messages
- [ ] Make the API more idiot-proof (eg look at error messages when a component is written incorrectly).
- [ ] Add even more tracing/logging.
- [ ] Write `#[derive(Event)]` macro.
- [ ] Write macro attribute for type-checking components.
- [ ] Refactor event system internals (keep API as-is).
- [ ] Add AnyEvent type.
- [ ] Rework doc
  - [ ] Improve discoverability
  - [ ] Add doc example of element_tuple instantiated with 3 args
  - [ ] Add test that checks that the README and the todo example are identical
  - [ ] Notes on the limitations of static typing, hence why we use Option and Either
  - [ ] Add "speedrun" doc
  - [ ] Explain event system
  - [ ] Explain the notion that some elements represent 0, 1 or n flex items
  - [ ] Add note that flex widget is both a flex item and a container
  - [ ] Use `#[doc = include_str!]` once feature gets stable
  - [ ] Document event chaining
  - [ ] Add screenshots to doc
- [ ] Add styling elements
  - [ ] Background and borders
  - [ ] Text font, style
  - [ ] Margins
- [ ] Remove some cloning
- [X] Remove component_caller
- [ ] Add WithKey type and `WidgetExt.with_key` method
- [ ] Implement two-way bindings
- [ ] Add Spacer element (from druid)
- [ ] Check out SizedBox (?)
- [ ] Refactor with_flex_params; use `From<f64>`??
- [ ] Add tests for identical update (eg `Label("Hello")` then `Label("Hello")` again)

Also, not a specific item but something that needs to be done continuously:

- Add new basic elements and useful components.

## Medium term

Some cool features I'd like:

- [ ] Add "free-form" mode, that can be integrated into an external main loop (eg for game engines).
- [ ] Implement screenshot generator for examples.
- [ ] Implement accessibility features.
- [ ] Add integration tests based on visual snapshots.

## Long term - Feature parity with React

TODO

## Long term - Debugging framework

TODO
