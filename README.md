# capitaine

This project is a prototype implementation of a virtual-DOM-based UI for the Rust programming language. It's inspired by [Crochet](https://github.com/raphlinus/crochet/) (hence the name) and [React](https://github.com/facebook/react), and implemented on top of [druid](https://github.com/linebender/druid).

The project is very much WIP right now.


## Architecture

Documentation is TODO.

The general idea is that the end-user writes pure functions (aka components) that return trees of GUI elements. Unlike other virtual-DOM libraries, in Capitaine these trees are statically typed, which allows us to skip a lot of type-related boilerplate.

Components have two parameters, whose type is static and user-defined: local state, which is stable for a given instantiation of the component, and props, which are passed by the parent component.

Inside a component, GUI elements may have an event handler attached to them, which can mutate the local component state.


## Contributing

The project is in an early stage and isn't too open to contributions for now, so, if you have a medium-to-large change in mind, you should probably open an issue to discuss it first.

Small fixes and typos corrections are welcome and will be merged quickly.
