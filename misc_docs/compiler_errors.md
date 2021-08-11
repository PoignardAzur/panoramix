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



// ----

 1  error[E0277]: the trait bound `&dyn widget_sequence::FlexWidget: widget_sequence::FlexWidget` is not satisfied
   --> src/widgets/clickable_widget.rs:33:9
    |
 33 |         children.first().unwrap()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `widget_sequence::FlexWidget` is not implemented for `&dyn widget_sequence::FlexWidget`
    |
    = note: required for the cast to the object type `dyn widget_sequence::FlexWidget`

Trying to use &&dyn FlexWidget instead of &dyn FlexWidget
