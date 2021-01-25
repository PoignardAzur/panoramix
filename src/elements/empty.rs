use crate::element_tree::{ElementTree, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;
use crate::widgets::EmptySequence;

use derivative::Derivative;

#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""))]
pub struct EmptyElement<ComponentState = (), ComponentEvent = NoEvent>(
    pub std::marker::PhantomData<ComponentState>,
    pub std::marker::PhantomData<ComponentEvent>,
);

#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""))]
pub struct EmptyElementData<ComponentState = (), ComponentEvent = NoEvent>(
    pub std::marker::PhantomData<ComponentState>,
    pub std::marker::PhantomData<ComponentEvent>,
);

impl<ComponentState, ComponentEvent> EmptyElement<ComponentState, ComponentEvent> {
    pub fn new() -> EmptyElement<ComponentState, ComponentEvent> {
        EmptyElement(Default::default(), Default::default())
    }
}

impl<ComponentState, ComponentEvent> ElementTree<ComponentState, ComponentEvent>
    for EmptyElement<ComponentState, ComponentEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type BuildOutput = EmptyElementData<ComponentState, ComponentEvent>;

    fn build(self, _prev_state: ()) -> (EmptyElementData<ComponentState, ComponentEvent>, ()) {
        (EmptyElementData(Default::default(), Default::default()), ())
    }
}

impl<ComponentState, ComponentEvent> VirtualDom<ComponentState, ComponentEvent>
    for EmptyElementData<ComponentState, ComponentEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type TargetWidgetSeq = EmptySequence;

    fn update_value(&mut self, _other: Self) {}

    fn init_tree(&self) -> EmptySequence {
        EmptySequence
    }

    fn reconcile(&self, _other: &Self, _widget_seq: &mut EmptySequence) {}

    fn process_event(
        &self,
        _component_state: &mut ComponentState,
        _children_state: &mut (),
        _widget_seq: &mut EmptySequence,
        _cx: &mut GlobalEventCx,
    ) -> Option<NoEvent> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    #[test]
    fn new_empty() {
        let empty = EmptyElement::<()>::new();
        let (empty_data, _) = empty.clone().build(());
        assert_eq!(empty, EmptyElement(Default::default(), Default::default()));
        assert_eq!(
            empty_data,
            EmptyElementData(Default::default(), Default::default())
        );
    }

    // TODO
    // - Widget test
}
