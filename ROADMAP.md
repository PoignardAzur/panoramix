# Roadmap

## Short term

- [X] Implement the main logic.
 - [X] Virtual DOM constrution and reconciliation.
 - [X] Flex logic.
 - [X] Event logic.
- [ ] Write documentation.
 - [ ] Readme.
 - [ ] Top-level doc page.
 - [ ] Writing a component.
 - [ ] All symbols (types, traits, functions, etc).
- [ ] Write unit tests.
 - [X] Basic constructors.
 - [X] compute_diff algorithm.
 - [ ] Event handling
 - [ ] Widget mutation.
- [ ] Add TextBox element.
- [ ] Add testing backend to druid for unit tests.
- [ ] Add integration tests based on visual snapshots.
- [ ] Write some benchmarks.
- [ ] Add dynamically-typed BoxDynElement (probably using `std::Any`).
- [ ] Add immutable ConstElement.
- [ ] Refactor ElementList to allow duplicates
- [ ] Make the API more idiot-proof (eg look at error messages when a component is written incorrectly).
- [ ] Add even more tracing/logging.
- [ ] Write `#[derive(Event)]` macro.
- [ ] Write macro attribute for type-checking components.
- [ ] Refactor event system internals (keep API as-is).
- [ ] Add AnyEvent type.
- Use `#[doc = include_str!]` once feature gets stable

Also, not a specific item but something that needs to be done continuously:

- Add new basic elements and useful components.

## Medium term

Some cool features I'd like:

- [ ] Add "free-form" mode, that can be integrated into an external main loop (eg for game engines).
- [ ] Implement screenshot generator for examples.
- [ ] Implement accessibility features.

## Long term - Feature parity with React

TODO

## Long term - Debugging framework

TODO
