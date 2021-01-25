use crate::element_tree::{ElementTree, VirtualDom};
use crate::glue::GlobalEventCx;

use tracing::instrument;

pub struct WithEvent<
    ComponentState,
    ComponentEvent,
    Child: ElementTree<ComponentState, ComponentEvent>,
    Cb: Fn(&mut ComponentState, &Child::Event),
> {
    pub element: Child,
    pub callback: Cb,
    pub _comp_state: std::marker::PhantomData<ComponentState>,
    pub _comp_event: std::marker::PhantomData<ComponentEvent>,
}

pub struct WithEventTarget<
    ComponentState,
    ComponentEvent,
    Child: VirtualDom<ComponentState, ComponentEvent>,
    Cb: Fn(&mut ComponentState, &Child::Event),
> {
    element: Child,
    callback: Cb,
    _comp_state: std::marker::PhantomData<ComponentState>,
    _comp_event: std::marker::PhantomData<ComponentEvent>,
}

// ---

impl<
        ComponentState,
        ComponentEvent,
        Child: ElementTree<ComponentState, ComponentEvent>,
        Cb: Fn(
            &mut ComponentState,
            &Child::Event,
        ),
    > std::fmt::Debug for WithEvent<ComponentState, ComponentEvent, Child, Cb>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithEvent")
            .field("element", &self.element)
            .field("callback", &std::any::type_name::<Cb>())
            .finish()
    }
}

impl<
        ComponentState,
        ComponentEvent,
        Child: VirtualDom<ComponentState, ComponentEvent>,
        Cb: Fn(&mut ComponentState, &Child::Event),
    > std::fmt::Debug for WithEventTarget<ComponentState, ComponentEvent, Child, Cb>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithEvent")
            .field("element", &self.element)
            .field("callback", &std::any::type_name::<Cb>())
            .finish()
    }
}

impl<
        ComponentState,
        ComponentEvent,
        Child: ElementTree<ComponentState, ComponentEvent>,
        Cb: Fn(
            &mut ComponentState,
            &<Child::BuildOutput as VirtualDom<ComponentState, ComponentEvent>>::Event,
        ),
    > ElementTree<ComponentState, ComponentEvent>
    for WithEvent<ComponentState, ComponentEvent, Child, Cb>
{
    type Event = Child::Event;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = WithEventTarget<ComponentState, ComponentEvent, Child::BuildOutput, Cb>;

    #[instrument(name = "WithEvent", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (element, state) = self.element.build(prev_state);
        (
            WithEventTarget {
                element,
                callback: self.callback,
                _comp_state: Default::default(),
                _comp_event: Default::default(),
            },
            state,
        )
    }
}

impl<
        ComponentState,
        ComponentEvent,
        Child: VirtualDom<ComponentState, ComponentEvent>,
        Cb: Fn(&mut ComponentState, &Child::Event),
    > VirtualDom<ComponentState, ComponentEvent>
    for WithEventTarget<ComponentState, ComponentEvent, Child, Cb>
{
    type Event = Child::Event;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type TargetWidgetSeq = Child::TargetWidgetSeq;

    #[instrument(name = "WithEvent", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        self.element.update_value(other.element);
    }

    #[instrument(name = "WithEvent", skip(self))]
    fn init_tree(&self) -> Child::TargetWidgetSeq {
        self.element.init_tree()
    }

    #[instrument(name = "WithEvent", skip(self, other, widget_seq))]
    fn reconcile(&self, other: &Self, widget_seq: &mut Self::TargetWidgetSeq) {
        self.element.reconcile(&other.element, widget_seq)
    }

    #[instrument(
        name = "WithEvent",
        skip(self, component_state, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        component_state: &mut ComponentState,
        children_state: &mut Child::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<Child::Event> {
        let event = self
            .element
            .process_event(component_state, children_state, widget_seq, cx);
        if let Some(event) = event.as_ref() {
            (self.callback)(component_state, event);
        }
        event
    }
}

// Note - Tests related to with_event will be in component_caller.rs for now
