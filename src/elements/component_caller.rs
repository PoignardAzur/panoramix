use crate::glue::GlobalEventCx;

use crate::element_tree::{ElementTree, VirtualDom};

use derivative::Derivative;

pub struct ComponentCaller<
    CompExplicitState,
    Props,
    ReturnedTree: ElementTree<CompExplicitState>,
    Comp: Fn(&CompExplicitState, Props) -> ReturnedTree,
    ExplicitState = (),
> {
    pub component: Comp,
    pub props: Props,
    pub _state: std::marker::PhantomData<CompExplicitState>,
    pub _tree: std::marker::PhantomData<ReturnedTree>,
    pub _expl_state: std::marker::PhantomData<ExplicitState>,
}

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""))]
pub struct ComponentCallerData<
    ParentComponentState,
    ChildComponentState: Default,
    Child: VirtualDom<ChildComponentState>,
>(
    Child,
    std::marker::PhantomData<ParentComponentState>,
    std::marker::PhantomData<ChildComponentState>,
);

impl<
        ExplicitState,
        CompExplicitState,
        Props,
        ReturnedTree: ElementTree<CompExplicitState>,
        Comp: Fn(&CompExplicitState, Props) -> ReturnedTree,
    > ComponentCaller<CompExplicitState, Props, ReturnedTree, Comp, ExplicitState>
{
    pub fn prepare(
        component: Comp,
        props: Props,
    ) -> ComponentCaller<CompExplicitState, Props, ReturnedTree, Comp, ExplicitState> {
        ComponentCaller {
            component,
            props,
            _state: Default::default(),
            _tree: Default::default(),
            _expl_state: Default::default(),
        }
    }
}

impl<
        ExplicitState,
        CompExplicitState,
        Props,
        ReturnedTree: ElementTree<CompExplicitState>,
        Comp: Fn(&CompExplicitState, Props) -> ReturnedTree,
    > std::fmt::Debug
    for ComponentCaller<CompExplicitState, Props, ReturnedTree, Comp, ExplicitState>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentCaller")
            .field("component", &std::any::type_name::<Comp>())
            .field("props", &"<props>")
            .field("_state", &self._state)
            .field("_tree", &self._tree)
            .field("_expl_state", &self._expl_state)
            .finish()
    }
}

impl<
        ExplicitState,
        CompExplicitState: Default,
        Props,
        ReturnedTree: ElementTree<CompExplicitState>,
        Comp: Fn(&CompExplicitState, Props) -> ReturnedTree,
    > ElementTree<ExplicitState>
    for ComponentCaller<CompExplicitState, Props, ReturnedTree, Comp, ExplicitState>
{
    type Event = ReturnedTree::Event;
    type AggregateComponentState = (CompExplicitState, ReturnedTree::AggregateComponentState);
    type BuildOutput =
        ComponentCallerData<ExplicitState, CompExplicitState, ReturnedTree::BuildOutput>;

    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState) {
        let element_tree = (self.component)(&prev_state.0, self.props);
        let (element, component_state) = element_tree.build(prev_state.1);
        (
            ComponentCallerData(element, Default::default(), Default::default()),
            (prev_state.0, component_state),
        )
    }
}

impl<
        ParentComponentState,
        ChildComponentState: Default,
        Child: VirtualDom<ChildComponentState>,
    > VirtualDom<ParentComponentState>
    for ComponentCallerData<ParentComponentState, ChildComponentState, Child>
{
    type Event = Child::Event;
    type DomState = Child::DomState;
    type AggregateComponentState = (ChildComponentState, Child::AggregateComponentState);

    type TargetWidgetSeq = Child::TargetWidgetSeq;

    fn update_value(&mut self, other: Self) {
        self.0.update_value(other.0);
    }

    fn init_tree(&self) -> (Child::TargetWidgetSeq, Child::DomState) {
        self.0.init_tree()
    }

    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Child::DomState,
        widget: &mut Child::TargetWidgetSeq,
    ) -> Self::DomState {
        self.0.apply_diff(&other.0, prev_state, widget)
    }

    fn process_event(
        &self,
        _explicit_state: &mut ParentComponentState,
        children_state: &mut Self::AggregateComponentState,
        dom_state: &mut Self::DomState,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        self.0
            .process_event(&mut children_state.0, &mut children_state.1, dom_state, cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element_tree::assign_empty_state_type;
    use crate::element_tree_ext::ElementTreeExt;
    use crate::elements::Button;
    use crate::elements::ButtonPressed;
    use crate::elements::EventEnum;
    use crate::elements::Label;
    use crate::make_row;
    use insta::assert_debug_snapshot;

    type MyEvent = EventEnum<ButtonPressed, ()>;

    // TODO - add tracing, and detect when this function is called by tests
    fn my_component(state: &u16, props: i64) -> impl ElementTree<u16, Event = MyEvent> {
        make_row!(
            Button::new("Press me").with_event(|state: &mut u16, _| {
                *state += 1;
            }),
            Label::new(format!("Values: {} {}", state, props)),
        )
    }

    #[test]
    fn call_component() {
        let comp_caller = ComponentCaller::prepare(my_component, 16);
        assign_empty_state_type(&comp_caller);

        let prev_state = (999, Default::default());
        let (component_result, component_state) = comp_caller.build(prev_state);

        assert_eq!(component_state.0, 999);
        assert_debug_snapshot!(component_result);

        // TODO - process_event
    }

    // TODO
    // - Widget test
    // - Events
}
