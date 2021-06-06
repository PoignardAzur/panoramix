use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;
use crate::widgets::EmptySequence;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;

/// A placeholder element.
///
/// This does **not** represent a blank area or a zero-sized widget. Rather, this represents the *absence* of a widget. So, for instance:
///
/// ```no_compile
/// Column!(
///     Label::new("Hello world!"),
///     EmptyElement::new(),
/// )
/// ```
///
/// is equivalent to:
///
/// ```no_compile
/// Column!(
///     Label::new("Hello world!"),
/// )
/// ```
///
/// in terms of layout computation and everything else.
///
/// ## Events
///
/// Doesn't emit events.
#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""))]
pub struct EmptyElement<CpEvent = NoEvent, CpState = ()>(
    #[derivative(Debug = "ignore")] pub std::marker::PhantomData<CpState>,
    #[derivative(Debug = "ignore")] pub std::marker::PhantomData<CpEvent>,
);

#[derive(Derivative, Clone, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""))]
pub struct EmptyElementData<CpEvent = NoEvent, CpState = ()>(
    #[derivative(Debug = "ignore")] pub std::marker::PhantomData<CpState>,
    #[derivative(Debug = "ignore")] pub std::marker::PhantomData<CpEvent>,
);

impl<CpEvent, CpState> EmptyElement<CpEvent, CpState> {
    pub fn new() -> EmptyElement<CpEvent, CpState> {
        EmptyElement(Default::default(), Default::default())
    }
}

impl<CpEvent, CpState> Element<CpEvent, CpState> for EmptyElement<CpEvent, CpState> {
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type BuildOutput = EmptyElementData<CpEvent, CpState>;

    fn build(self, _prev_state: ()) -> (EmptyElementData<CpEvent, CpState>, ()) {
        (EmptyElementData(Default::default(), Default::default()), ())
    }
}

impl<CpEvent, CpState> VirtualDom<CpEvent, CpState> for EmptyElementData<CpEvent, CpState> {
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type TargetWidgetSeq = EmptySequence;

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
