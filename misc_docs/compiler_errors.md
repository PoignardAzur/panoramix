Error when swapping two parameters

```
Error[E0277]: the trait bound `State: element_tree::ElementTree<label::Label<State>>` is not satisfied
   --> src/elements/element_list.rs:186:49
    |
10  | pub struct ElementList<Child: ElementTree<ComponentState>, ComponentState = ()> {
    |                               -------------------------- required by this bound in `element_list::ElementList`
...
186 |     fn new_label_list<State>(names: &[&str]) -> ElementList<State, Label<State>> {
    |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `element_tree::ElementTree<label::Label<State>>` is not implemented for `State`
    |
help: consider restricting type parameter `State`
    |
186 |     fn new_label_list<State: element_tree::ElementTree<label::Label<State>>>(names: &[&str]) -> ElementList<State, Label<State>> {
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

A better error message would be

```
help: consider swapping template arguments
```
