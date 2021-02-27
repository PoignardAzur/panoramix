use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;
use crate::widgets::EmptySequence;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;

#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""))]
pub struct EmptyElement<CpState = (), CpEvent = NoEvent>(
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""))]
pub struct EmptyElementData<CpState = (), CpEvent = NoEvent>(
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

impl<CpState, CpEvent> EmptyElement<CpState, CpEvent> {
    pub fn new() -> EmptyElement<CpState, CpEvent> {
        EmptyElement(Default::default(), Default::default())
    }
}

impl<CpState, CpEvent> Element<CpState, CpEvent> for EmptyElement<CpState, CpEvent> {
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type BuildOutput = EmptyElementData<CpState, CpEvent>;

    fn build(self, _prev_state: ()) -> (EmptyElementData<CpState, CpEvent>, ()) {
        (EmptyElementData(Default::default(), Default::default()), ())
    }
}

impl<CpState, CpEvent> VirtualDom<CpState, CpEvent> for EmptyElementData<CpState, CpEvent> {
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type TargetWidgetSeq = EmptySequence;

    fn update_value(&mut self, _other: Self) {}

    fn init_tree(&self) -> EmptySequence {
        EmptySequence
    }

    fn reconcile(&self, _other: &Self, _widget_seq: &mut EmptySequence, _ctx: &mut ReconcileCtx) {}

    fn process_event(
        &self,
        _component_state: &mut CpState,
        _children_state: &mut (),
        _widget_seq: &mut EmptySequence,
        _cx: &mut GlobalEventCx,
    ) -> Option<CpEvent> {
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
