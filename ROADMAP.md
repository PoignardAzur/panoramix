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
- [X] Write tests.
  - [X] Basic constructors.
  - [X] compute_diff algorithm.
  - [X] Event handling.
  - [X] Widget mutation.
  - [X] Add integration test using all components, and snapshot-test the return values of every method.
  - [X] Add integration test for use_metadata and get_local_state.
- [X] Add TextBox element.
- [X] Add testing backend to druid for unit tests.
- [X] Add dynamically-typed BoxDynElement (probably using `std::Any`).
- [X] Rework MockState.
- [X] Fix component macro error messages.
- [X] Make the API more idiot-proof (eg look at error messages when a component is written incorrectly).
- [X] Remove component_caller.
- [ ] Refactor ElementList
  - [ ] Allow duplicate keys.
  - [ ] Refactor compute_diff (look up diff algorithms?).
- [ ] Refactor event handling.
  - [ ] Rework internals to allow event chaining, improve performance.
  - [ ] Test event chaining.
  - [ ] Write `#[derive(Event)]` macro.
  - [ ] Add Event trait.
  - [ ] Add AnyEvent type.
- [ ] Rework doc.
  - [X] Improve discoverability.
  - [X] Add doc example of element_tuple instantiated with 3 args.
  - [X] Add test that checks that the README and the todo example are identical.
  - [X] Rewrite tutorials.
  - [X] Rewrite Element doc.
    - [X] Explain event system.
    - [X] Document ElementExt.
  - [ ] Add "speedrun" doc.
  - [X] Use `#[doc = include_str!]` once feature gets stable.
  - [X] Move tutorials to inline doc.
  - [ ] Notes on the limitations of static typing, hence why we use Option and Either.
  - [ ] Explain ParentEvent system.
  - [ ] Explain the notion that some elements represent 0, 1 or n flex items.
  - [ ] Add note that flex widget is both a flex item and a container.
  - [ ] Document event chaining.
  - [ ] Add screenshots to doc.
  - [ ] Rewrite ARCHITECTURE.md
- [ ] Add styling elements.
  - [ ] Background and borders.
  - [ ] Text font, style.
  - [ ] Margins.
  - [ ] Add Spacer element (from druid).
  - [ ] Refactor with_flex_params; use `From<f64>`??
- [ ] Remove some cloning.
- [ ] Add WithKey type and `WidgetExt.with_key` method.
- [ ] Implement two-way bindings.
- [ ] Check out SizedBox (?).
- [ ] Have MockComponent actually implement Component.
- [ ] Add tests for identical update (eg `Label("Hello")` then `Label("Hello")` again).
- [ ] Write some benchmarks.

Also, not a specific item but something that needs to be done continuously:

- Add new basic elements and useful components.

## Medium term

Some cool features I'd like:

- [ ] Add "free-form" mode, that can be integrated into an external main loop (eg for game engines).
- [ ] Implement screenshot generator for examples.
- [ ] Implement accessibility features.
- [ ] Add integration tests based on visual snapshots.
- [ ] Add even more tracing/logging.

## Long term - Feature parity with React

TODO

## Long term - Debugging framework

TODO
