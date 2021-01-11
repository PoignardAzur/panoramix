use crate::glue::GlobalEventCx;
use crate::widgets::EmptySequence;

use crate::element_tree::{ElementTree, VirtualDom};

use derivative::Derivative;

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""))]
pub struct EmptyElement<ExplicitState = ()>(pub std::marker::PhantomData<ExplicitState>);

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""))]
pub struct EmptyElementData<ParentComponentState>(
    pub std::marker::PhantomData<ParentComponentState>,
);

impl<ExplicitState> EmptyElement<ExplicitState> {
    pub fn new() -> EmptyElement<ExplicitState> {
        EmptyElement(Default::default())
    }
}

impl<ExplicitState> ElementTree<ExplicitState> for EmptyElement<ExplicitState> {
    type Event = ();
    type AggregateComponentState = ();
    type BuildOutput = EmptyElementData<ExplicitState>;

    fn build(self, _prev_state: ()) -> (EmptyElementData<ExplicitState>, ()) {
        (EmptyElementData(Default::default()), ())
    }
}

impl<ParentComponentState> VirtualDom<ParentComponentState>
    for EmptyElementData<ParentComponentState>
{
    type Event = ();
    type DomState = ();
    type AggregateComponentState = ();

    type TargetWidgetSeq = EmptySequence;

    fn update_value(&mut self, _other: Self) {}

    fn init_tree(&self) -> (Self::TargetWidgetSeq, ()) {
        (EmptySequence, ())
    }

    fn apply_diff(
        &self,
        _other: &Self,
        _prev_state: (),
        _widget: &mut Self::TargetWidgetSeq,
    ) -> () {
    }

    fn process_event(
        &self,
        _explicit_state: &mut ParentComponentState,
        _children_state: &mut (),
        _dom_state: &mut (),
        _cx: &mut GlobalEventCx,
    ) -> Option<()> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let empty = EmptyElement::<()>::new();
        let (empty_data, _) = empty.clone().build(());
        assert_eq!(empty, EmptyElement(Default::default()));
        assert_eq!(empty_data, EmptyElementData(Default::default()));
    }

    // TODO
    // - Widget test
}
