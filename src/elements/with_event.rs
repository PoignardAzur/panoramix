use crate::element_tree::{Element, Metadata, VirtualDom};
use crate::glue::GlobalEventCx;

use crate::element_tree::ProcessEventCtx;
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
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct WithCallbackEvent<
    CpEvent,
    CpState,
    EventParam,
    Child: Element<CpEvent, CpState>,
    Cb: Fn(&mut CpState, EventParam) + Clone,
> where
    Child::Event: ParentEvent<EventParam>,
{
    pub element: Child,
    #[derivative(Debug(format_with = "format_typename"))]
    pub callback: Cb,
    #[derivative(Debug = "ignore")]
    pub _metadata: Metadata<CpEvent, CpState>,
    #[derivative(Debug = "ignore")]
    pub _marker: std::marker::PhantomData<EventParam>,
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct WithMapEvent<
    CpEvent,
    CpState,
    EventParam,
    EventReturn,
    Child: Element<CpEvent, CpState>,
    Cb: Fn(&mut CpState, EventParam) -> Option<EventReturn> + Clone,
> where
    Child::Event: ParentEvent<EventParam>,
    CpEvent: ParentEvent<EventReturn>,
{
    pub element: Child,
    #[derivative(Debug(format_with = "format_typename"))]
    pub callback: Cb,
    #[derivative(Debug = "ignore")]
    pub _metadata: Metadata<CpEvent, CpState>,
    #[derivative(Debug = "ignore")]
    pub _marker: std::marker::PhantomData<EventParam>,
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct WithBubbleEvent<CpEvent, CpState, Event, Child: Element<CpEvent, CpState>>
where
    Child::Event: ParentEvent<Event>,
    CpEvent: ParentEvent<Event>,
{
    pub element: Child,
    #[derivative(Debug = "ignore")]
    pub _metadata: Metadata<CpEvent, CpState>,
    #[derivative(Debug = "ignore")]
    pub _marker: std::marker::PhantomData<Event>,
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct WithEventTarget<
    CpEvent,
    CpState,
    EventParam,
    EventReturn,
    CbReturn: OptionOrUnit<EventReturn>,
    Child: VirtualDom<CpEvent, CpState>,
    Cb: Fn(&mut CpState, EventParam) -> CbReturn + Clone,
> where
    Child::Event: ParentEvent<EventParam>,
    CpEvent: ParentEvent<EventReturn>,
{
    element: Child,
    #[derivative(Debug(format_with = "format_typename"))]
    callback: Cb,
    #[derivative(Debug = "ignore")]
    _metadata: Metadata<CpEvent, CpState>,
    #[derivative(Debug = "ignore")]
    _marker: std::marker::PhantomData<(EventParam, EventReturn)>,
}

// ---

impl<
        CpEvent,
        CpState,
        EventParam,
        Child: Element<CpEvent, CpState>,
        Cb: Fn(&mut CpState, EventParam) + Clone,
    > Element<CpEvent, CpState> for WithCallbackEvent<CpEvent, CpState, EventParam, Child, Cb>
where
    Child::Event: ParentEvent<EventParam>,
{
    type Event = Child::Event;
    type ComponentState = crate::element_tree::NoState;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput =
        WithEventTarget<CpEvent, CpState, EventParam, CpEvent, (), Child::BuildOutput, Cb>;

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
                _metadata: Default::default(),
                _marker: Default::default(),
            },
            state,
        )
    }
}

impl<
        CpEvent,
        CpState,
        EventParam,
        EventReturn,
        Child: Element<CpEvent, CpState>,
        Cb: Fn(&mut CpState, EventParam) -> Option<EventReturn> + Clone,
    > Element<CpEvent, CpState>
    for WithMapEvent<CpEvent, CpState, EventParam, EventReturn, Child, Cb>
where
    Child::Event: ParentEvent<EventParam>,
    CpEvent: ParentEvent<EventReturn>,
{
    type Event = Child::Event;
    type ComponentState = crate::element_tree::NoState;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = WithEventTarget<
        CpEvent,
        CpState,
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
                _metadata: Default::default(),
                _marker: Default::default(),
            },
            state,
        )
    }
}

impl<CpEvent, CpState, Event, Child: Element<CpEvent, CpState>> Element<CpEvent, CpState>
    for WithBubbleEvent<CpEvent, CpState, Event, Child>
where
    Child::Event: ParentEvent<Event>,
    CpEvent: ParentEvent<Event>,
{
    type Event = Child::Event;

    type ComponentState = crate::element_tree::NoState;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = WithEventTarget<
        CpEvent,
        CpState,
        Event,
        Event,
        Option<Event>,
        Child::BuildOutput,
        fn(&mut CpState, Event) -> Option<Event>,
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
                _metadata: Default::default(),
                _marker: Default::default(),
            },
            state,
        )
    }
}

impl<
        CpEvent,
        CpState,
        EventParam,
        EventReturn,
        CbReturn: OptionOrUnit<EventReturn>,
        Child: VirtualDom<CpEvent, CpState>,
        Cb: Fn(&mut CpState, EventParam) -> CbReturn + Clone,
    > VirtualDom<CpEvent, CpState>
    for WithEventTarget<CpEvent, CpState, EventParam, EventReturn, CbReturn, Child, Cb>
where
    Child::Event: ParentEvent<EventParam>,
    CpEvent: ParentEvent<EventReturn>,
{
    type Event = Child::Event;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type TargetWidgetSeq = Child::TargetWidgetSeq;

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
        skip(self, comp_ctx, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx<CpEvent, CpState>,
        children_state: &mut Child::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        // First, recursively handle all child events
        self.element
            .process_event(comp_ctx, children_state, widget_seq, cx);

        // FIXME - Handle chains of callbacks eg
        /*
            Button(...)
                .on(|Pressed| ...)
                .on(|MouseEnter| ...)
                .on(|MouseLeave| ...)
        */

        let local_event = self
            .element
            .process_local_event(children_state, widget_seq, cx);
        if let Some(local_event) = local_event.map(ParentEvent::into_child_event).flatten() {
            trace!("Processing callback for local event");
            let event = (self.callback)(comp_ctx.state, local_event)
                .to_option()
                .map(CpEvent::from_child_event);
            if let Some(event) = event {
                // TODO - Log event
                trace!("Callback returned event");
                comp_ctx.event_queue.push(event);
            }
        }
    }
}

// Note - Tests related to with_event will be in component_caller.rs for now
