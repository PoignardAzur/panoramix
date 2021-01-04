//! A prototype implementation of reactive UI in rust
//!
//! This project is a prototype implementation of a virtual-DOM-based UI for the Rust programming
//! language. It's inspired by [Crochet](https://github.com/raphlinus/crochet/) (hence the name) and
//! [React](https://github.com/facebook/react), and implemented on top of
//! [druid](https://github.com/linebender/druid).

pub mod element_tree;
pub mod element_tree_ext;
pub mod elements;
pub mod flex2;
pub mod glue;
pub mod root_handler;
pub mod widget_sequence;
pub mod widgets;
