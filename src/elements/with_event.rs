use crate::glue::GlobalEventCx;

use crate::element_tree::{ElementTree, VirtualDom};

pub struct WithEvent<
    Child: ElementTree<ExplicitState>,
    Cb: Fn(&mut ExplicitState, &<Child::BuildOutput as VirtualDom<ExplicitState>>::Event),
    ExplicitState = (),
> {
    pub element: Child,
    pub callback: Cb,
    pub _state: std::marker::PhantomData<ExplicitState>,
}

pub struct WithEventTarget<
    Child: VirtualDom<ExplicitState>,
    Cb: Fn(&mut ExplicitState, &Child::Event),
    ExplicitState,
> {
    element: Child,
    callback: Cb,
    _state: std::marker::PhantomData<ExplicitState>,
}

impl<
        Child: VirtualDom<ParentComponentState>,
        Cb: Fn(&mut ParentComponentState, &Child::Event),
        ParentComponentState,
    > VirtualDom<ParentComponentState> for WithEventTarget<Child, Cb, ParentComponentState>
{
    type Event = Child::Event;
    type DomState = Child::DomState;
    type AggregateComponentState = Child::AggregateComponentState;

    type TargetWidgetSeq = Child::TargetWidgetSeq;

    fn update_value(&mut self, other: Self) {
        self.element.update_value(other.element);
    }

    fn init_tree(&self) -> (Child::TargetWidgetSeq, Child::DomState) {
        self.element.init_tree()
    }

    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Child::DomState,
        widget: &mut Self::TargetWidgetSeq,
    ) -> Child::DomState {
        self.element.apply_diff(&other.element, prev_state, widget)
    }

    fn process_event(
        &self,
        explicit_state: &mut ParentComponentState,
        children_state: &mut Child::AggregateComponentState,
        dom_state: &mut Child::DomState,
        cx: &mut GlobalEventCx,
    ) -> Option<Child::Event> {
        let event = self
            .element
            .process_event(explicit_state, children_state, dom_state, cx);
        if let Some(event) = event.as_ref() {
            (self.callback)(explicit_state, event);
        }
        event
    }
}

impl<
        Child: ElementTree<ExplicitState>,
        ExplicitState,
        Cb: Fn(&mut ExplicitState, &<Child::BuildOutput as VirtualDom<ExplicitState>>::Event),
    > ElementTree<ExplicitState> for WithEvent<Child, Cb, ExplicitState>
{
    type Event = Child::Event;
    type AggregateComponentState = Child::AggregateComponentState;
    type BuildOutput = WithEventTarget<Child::BuildOutput, Cb, ExplicitState>;

    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState) {
        let (element, state) = self.element.build(prev_state);
        (
            WithEventTarget {
                element,
                callback: self.callback,
                _state: Default::default(),
            },
            state,
        )
    }
}
