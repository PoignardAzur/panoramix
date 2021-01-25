use crate::element_tree::{ElementTree, VirtualDom};
use crate::glue::GlobalEventCx;

use tracing::instrument;

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

// ---

impl<
        Child: ElementTree<ExplicitState>,
        ExplicitState,
        Cb: Fn(&mut ExplicitState, &<Child::BuildOutput as VirtualDom<ExplicitState>>::Event),
    > std::fmt::Debug for WithEvent<Child, Cb, ExplicitState>
where
    Child: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithEvent")
            .field("element", &self.element)
            .field("callback", &std::any::type_name::<Cb>())
            .field("_state", &self._state)
            .finish()
    }
}

impl<
        Child: VirtualDom<ParentComponentState>,
        Cb: Fn(&mut ParentComponentState, &Child::Event),
        ParentComponentState,
    > std::fmt::Debug for WithEventTarget<Child, Cb, ParentComponentState>
where
    Child: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithEvent")
            .field("element", &self.element)
            .field("callback", &std::any::type_name::<Cb>())
            .field("_state", &self._state)
            .finish()
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

    #[instrument(name = "WithEvent", skip(self, prev_state))]
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

    #[instrument(name = "WithEvent", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        self.element.update_value(other.element);
    }

    #[instrument(name = "WithEvent", skip(self))]
    fn init_tree(&self) -> (Child::TargetWidgetSeq, Child::DomState) {
        self.element.init_tree()
    }

    #[instrument(name = "WithEvent", skip(self, other, prev_state, widget))]
    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Child::DomState,
        widget: &mut Self::TargetWidgetSeq,
    ) -> Child::DomState {
        self.element.apply_diff(&other.element, prev_state, widget)
    }

    #[instrument(
        name = "WithEvent",
        skip(self, explicit_state, children_state, dom_state, cx)
    )]
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

// Note - Tests related to with_event will be in component_caller.rs for now
