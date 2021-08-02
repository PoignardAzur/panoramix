use crate::ctx::{ProcessEventCtx, ReconcileCtx};
use crate::element_tree::{Element, VirtualDom};
use crate::glue::GlobalEventCx;
use crate::metadata::{Metadata, NoState};

use derivative::Derivative;
use tracing::{instrument, trace};

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

/// TODO - Document this
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

/// Applies callback to events of child element
///
/// For internal use only. Library users should use [ElementExt](crate::ElementExt) instead.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct WithCallbackEvent<
    ComponentEvent: 'static,
    ComponentState: 'static,
    EventParam,
    Child: Element,
    Cb: Clone + Fn(&mut ComponentState, EventParam),
> where
    Child::Event: ParentEvent<EventParam>,
{
    pub element: Child,
    #[derivative(Debug(format_with = "format_typename"))]
    pub callback: Cb,
    #[derivative(Debug = "ignore")]
    pub _metadata: Metadata<ComponentEvent, ComponentState>,
    #[derivative(Debug = "ignore")]
    pub _marker: std::marker::PhantomData<EventParam>,
}

/// Maps events of child element into events of parent component, using provided map function.
///
/// For internal use only. Library users should use [ElementExt](crate::ElementExt) instead.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct WithMapEvent<
    ComponentEvent: 'static,
    ComponentState: 'static,
    EventParam,
    EventReturn,
    Child: Element,
    Cb: Clone + Fn(&mut ComponentState, EventParam) -> Option<EventReturn>,
> where
    Child::Event: ParentEvent<EventParam>,
    ComponentEvent: ParentEvent<EventReturn>,
{
    pub element: Child,
    #[derivative(Debug(format_with = "format_typename"))]
    pub callback: Cb,
    #[derivative(Debug = "ignore")]
    pub _metadata: Metadata<ComponentEvent, ComponentState>,
    #[derivative(Debug = "ignore")]
    pub _marker: std::marker::PhantomData<EventParam>,
}

/// Transfers events of child element to parent component.
///
/// For internal use only. Library users should use [ElementExt](crate::ElementExt) instead.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct WithBubbleEvent<ComponentEvent, ComponentState, Event, Child: Element>
where
    Child::Event: ParentEvent<Event>,
    ComponentEvent: ParentEvent<Event>,
{
    pub element: Child,
    #[derivative(Debug = "ignore")]
    pub _metadata: Metadata<ComponentEvent, ComponentState>,
    #[derivative(Debug = "ignore")]
    pub _marker: std::marker::PhantomData<Event>,
}

#[derive(Derivative)]
#[derivative(Clone(bound = "Child: Clone"), Debug(bound = ""))]
pub struct WithEventTarget<
    ComponentEvent: 'static,
    ComponentState: 'static,
    EventParam,
    EventReturn,
    CbReturn: OptionOrUnit<EventReturn>,
    Child: VirtualDom,
    Cb: Clone + Fn(&mut ComponentState, EventParam) -> CbReturn,
> where
    Child::Event: ParentEvent<EventParam>,
    ComponentEvent: ParentEvent<EventReturn>,
{
    element: Child,
    #[derivative(Debug(format_with = "format_typename"))]
    callback: Cb,
    #[derivative(Debug = "ignore")]
    _metadata: Metadata<ComponentEvent, ComponentState>,
    #[derivative(Debug = "ignore")]
    _marker: std::marker::PhantomData<(EventParam, EventReturn)>,
}

// ---

impl<
        ComponentEvent: 'static,
        ComponentState: 'static,
        EventParam: 'static,
        Child: Element,
        Cb: Clone + Fn(&mut ComponentState, EventParam) + 'static,
    > Element for WithCallbackEvent<ComponentEvent, ComponentState, EventParam, Child, Cb>
where
    Child::Event: ParentEvent<EventParam>,
{
    type Event = Child::Event;
    type ComponentState = NoState;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = WithEventTarget<
        ComponentEvent,
        ComponentState,
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
                _metadata: Default::default(),
                _marker: Default::default(),
            },
            state,
        )
    }
}

impl<
        ComponentEvent: 'static,
        ComponentState: 'static,
        EventParam: 'static,
        EventReturn: 'static,
        Child: Element,
        Cb: Clone + Fn(&mut ComponentState, EventParam) -> Option<EventReturn> + 'static,
    > Element for WithMapEvent<ComponentEvent, ComponentState, EventParam, EventReturn, Child, Cb>
where
    Child::Event: ParentEvent<EventParam>,
    ComponentEvent: ParentEvent<EventReturn>,
{
    type Event = Child::Event;
    type ComponentState = NoState;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = WithEventTarget<
        ComponentEvent,
        ComponentState,
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

impl<ComponentEvent: 'static, ComponentState: 'static, Event: 'static, Child: Element> Element
    for WithBubbleEvent<ComponentEvent, ComponentState, Event, Child>
where
    Child::Event: ParentEvent<Event>,
    ComponentEvent: ParentEvent<Event>,
{
    type Event = Child::Event;

    type ComponentState = NoState;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = WithEventTarget<
        ComponentEvent,
        ComponentState,
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
                _metadata: Default::default(),
                _marker: Default::default(),
            },
            state,
        )
    }
}

impl<
        ComponentEvent: 'static,
        ComponentState: 'static,
        EventParam,
        EventReturn,
        CbReturn: OptionOrUnit<EventReturn>,
        Child: VirtualDom,
        Cb: Clone + Fn(&mut ComponentState, EventParam) -> CbReturn,
    > VirtualDom
    for WithEventTarget<
        ComponentEvent,
        ComponentState,
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

    #[instrument(name = "WithEvent", skip(self))]
    fn init_tree(&self) -> Child::TargetWidgetSeq {
        self.element.init_tree()
    }

    #[instrument(name = "WithEvent", skip(self, prev_value, widget_seq, ctx))]
    fn reconcile(
        &self,
        prev_value: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.element.reconcile(&prev_value.element, widget_seq, ctx)
    }

    #[instrument(
        name = "WithEvent",
        skip(self, comp_ctx, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
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

        let md = self._metadata;
        let local_event = self
            .element
            .process_local_event(children_state, widget_seq, cx);
        if let Some(local_event) = local_event.map(ParentEvent::into_child_event).flatten() {
            trace!("Processing callback for local event");
            let event = (self.callback)(comp_ctx.state(md), local_event)
                .to_option()
                .map(ComponentEvent::from_child_event);
            if let Some(event) = event {
                // TODO - Log event
                trace!("Callback returned event");
                comp_ctx.event_queue(md).push(event);
            }
        }
    }
}

// Note - Tests related to with_event will be in component_caller.rs for now
