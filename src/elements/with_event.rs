use crate::element_tree::{ElementTree, VirtualDom};
use crate::glue::GlobalEventCx;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use tracing::{instrument, trace};

/*
TODO - Revisit names
- EventParam
- WithEventTarget
_comp_param
_comp_return

*/

// UTILITY TRAITS ---

pub trait OptionOrUnit<T> {
    fn to_option(self) -> Option<T>;
}

impl<T> OptionOrUnit<T> for Option<T> {
    fn to_option(self) -> Option<T> {
        self
    }
}

impl<T> OptionOrUnit<T> for () {
    fn to_option(self) -> Option<T> {
        None
    }
}

pub trait ParentEvent<Child> {
    fn from_child_event(child: Child) -> Self;
    fn into_child_event(self) -> Option<Child>;
}

impl<Parent, Child> ParentEvent<Child> for Parent
where
    Parent: std::convert::TryInto<Child>,
    Parent: std::convert::From<Child>,
{
    fn from_child_event(child: Child) -> Self {
        Self::from(child)
    }

    fn into_child_event(self) -> Option<Child> {
        self.try_into().ok()
    }
}

fn format_typename<T>(_value: &T, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    f.write_fmt(format_args!("{}", &std::any::type_name::<T>()))
}

fn bubble_event_up<State, Event>(_state: &mut State, event: Event) -> Option<Event> {
    Some(event)
}

// ---

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct WithCallbackEvent<
    ComponentState,
    ComponentEvent,
    EventParam,
    Child: ElementTree<ComponentState, ComponentEvent>,
    Cb: Fn(&mut ComponentState, EventParam),
> where
    Child::Event: ParentEvent<EventParam>,
{
    pub element: Child,
    #[derivative(Debug(format_with = "format_typename"))]
    pub callback: Cb,
    #[derivative(Debug = "ignore")]
    pub _comp_state: std::marker::PhantomData<ComponentState>,
    #[derivative(Debug = "ignore")]
    pub _comp_event: std::marker::PhantomData<ComponentEvent>,
    #[derivative(Debug = "ignore")]
    pub _comp_param: std::marker::PhantomData<EventParam>,
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct WithMapEvent<
    ComponentState,
    ComponentEvent,
    EventParam,
    EventReturn,
    Child: ElementTree<ComponentState, ComponentEvent>,
    Cb: Fn(&mut ComponentState, EventParam) -> Option<EventReturn>,
> where
    Child::Event: ParentEvent<EventParam>,
    ComponentEvent: ParentEvent<EventReturn>,
{
    pub element: Child,
    #[derivative(Debug(format_with = "format_typename"))]
    pub callback: Cb,
    #[derivative(Debug = "ignore")]
    pub _comp_state: std::marker::PhantomData<ComponentState>,
    #[derivative(Debug = "ignore")]
    pub _comp_event: std::marker::PhantomData<ComponentEvent>,
    #[derivative(Debug = "ignore")]
    pub _comp_param: std::marker::PhantomData<EventParam>,
    #[derivative(Debug = "ignore")]
    pub _comp_return: std::marker::PhantomData<EventParam>,
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct WithBubbleEvent<
    ComponentState,
    ComponentEvent,
    Event,
    Child: ElementTree<ComponentState, ComponentEvent>,
> where
    Child::Event: ParentEvent<Event>,
    ComponentEvent: ParentEvent<Event>,
{
    pub element: Child,
    #[derivative(Debug = "ignore")]
    pub _comp_state: std::marker::PhantomData<ComponentState>,
    #[derivative(Debug = "ignore")]
    pub _comp_event: std::marker::PhantomData<ComponentEvent>,
    #[derivative(Debug = "ignore")]
    pub _comp_param: std::marker::PhantomData<Event>,
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct WithEventTarget<
    ComponentState,
    ComponentEvent,
    EventParam,
    EventReturn,
    CbReturn: OptionOrUnit<EventReturn>,
    Child: VirtualDom<ComponentState, ComponentEvent>,
    Cb: Fn(&mut ComponentState, EventParam) -> CbReturn,
> where
    Child::Event: ParentEvent<EventParam>,
    ComponentEvent: ParentEvent<EventReturn>,
{
    element: Child,
    #[derivative(Debug(format_with = "format_typename"))]
    callback: Cb,
    #[derivative(Debug = "ignore")]
    _comp_state: std::marker::PhantomData<ComponentState>,
    #[derivative(Debug = "ignore")]
    _comp_event: std::marker::PhantomData<ComponentEvent>,
    #[derivative(Debug = "ignore")]
    _comp_param: std::marker::PhantomData<EventParam>,
    #[derivative(Debug = "ignore")]
    _comp_return: std::marker::PhantomData<EventReturn>,
}

// ---

impl<
        ComponentState,
        ComponentEvent,
        EventParam,
        Child: ElementTree<ComponentState, ComponentEvent>,
        Cb: Fn(&mut ComponentState, EventParam),
    > ElementTree<ComponentState, ComponentEvent>
    for WithCallbackEvent<ComponentState, ComponentEvent, EventParam, Child, Cb>
where
    Child::Event: ParentEvent<EventParam>,
{
    type Event = Child::Event;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = WithEventTarget<
        ComponentState,
        ComponentEvent,
        EventParam,
        ComponentEvent,
        (),
        Child::BuildOutput,
        Cb,
    >;

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
                _comp_param: Default::default(),
                _comp_return: Default::default(),
            },
            state,
        )
    }
}

impl<
        ComponentState,
        ComponentEvent,
        EventParam,
        EventReturn,
        Child: ElementTree<ComponentState, ComponentEvent>,
        Cb: Fn(&mut ComponentState, EventParam) -> Option<EventReturn>,
    > ElementTree<ComponentState, ComponentEvent>
    for WithMapEvent<ComponentState, ComponentEvent, EventParam, EventReturn, Child, Cb>
where
    Child::Event: ParentEvent<EventParam>,
    ComponentEvent: ParentEvent<EventReturn>,
{
    type Event = Child::Event;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = WithEventTarget<
        ComponentState,
        ComponentEvent,
        EventParam,
        EventReturn,
        Option<EventReturn>,
        Child::BuildOutput,
        Cb,
    >;

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
                _comp_param: Default::default(),
                _comp_return: Default::default(),
            },
            state,
        )
    }
}

impl<ComponentState, ComponentEvent, Event, Child: ElementTree<ComponentState, ComponentEvent>>
    ElementTree<ComponentState, ComponentEvent>
    for WithBubbleEvent<ComponentState, ComponentEvent, Event, Child>
where
    Child::Event: ParentEvent<Event>,
    ComponentEvent: ParentEvent<Event>,
{
    type Event = Child::Event;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = WithEventTarget<
        ComponentState,
        ComponentEvent,
        Event,
        Event,
        Option<Event>,
        Child::BuildOutput,
        fn(&mut ComponentState, Event) -> Option<Event>,
    >;

    #[instrument(name = "WithEvent", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (element, state) = self.element.build(prev_state);
        (
            WithEventTarget {
                element,
                callback: bubble_event_up,
                _comp_state: Default::default(),
                _comp_event: Default::default(),
                _comp_param: Default::default(),
                _comp_return: Default::default(),
            },
            state,
        )
    }
}

impl<
        ComponentState,
        ComponentEvent,
        EventParam,
        EventReturn,
        CbReturn: OptionOrUnit<EventReturn>,
        Child: VirtualDom<ComponentState, ComponentEvent>,
        Cb: Fn(&mut ComponentState, EventParam) -> CbReturn,
    > VirtualDom<ComponentState, ComponentEvent>
    for WithEventTarget<
        ComponentState,
        ComponentEvent,
        EventParam,
        EventReturn,
        CbReturn,
        Child,
        Cb,
    >
where
    Child::Event: ParentEvent<EventParam>,
    ComponentEvent: ParentEvent<EventReturn>,
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

    #[instrument(name = "WithEvent", skip(self, other, widget_seq, ctx))]
    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.element.reconcile(&other.element, widget_seq, ctx)
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
    ) -> Option<ComponentEvent> {
        // FIXME - Handle chains of callbacks eg
        /*
            Button(...)
                .on(|Pressed| ...)
                .on(|MouseEnter| ...)
                .on(|MouseLeave| ...)
        */
        let event = self
            .element
            .process_event(component_state, children_state, widget_seq, cx);
        if let Some(event) = event {
            trace!("Returned child event");
            return Some(event);
        }
        let local_event =
            self.element
                .process_local_event(component_state, children_state, widget_seq, cx);
        if let Some(local_event) = local_event.map(ParentEvent::into_child_event).flatten() {
            trace!("Processing callback for local event");
            let event = (self.callback)(component_state, local_event)
                .to_option()
                .map(ComponentEvent::from_child_event);
            if event.is_some() {
                trace!("Callback returned event");
            }
            return event;
        }
        None
    }
}

// Note - Tests related to with_event will be in component_caller.rs for now
