use crate::glue::{DruidAppData, GlobalEventCx};
use crate::widget_sequence::WidgetSequence;

use druid::{Env, EventCtx};
use std::any::Any;
use std::fmt::Debug;

pub struct CompCtx {
    pub(crate) local_state: Box<dyn Any>,
}

impl CompCtx {
    pub fn use_local_state<T: 'static>(&self) -> &T {
        self.local_state.downcast_ref::<T>().unwrap()
    }
}

/// Context required by [VirtualDom.reconcile]
pub struct ReconcileCtx<'a, 'b, 'c, 'd, 'e> {
    pub event_ctx: &'a mut EventCtx<'d, 'e>,
    pub data: &'b mut DruidAppData,
    pub env: &'c Env,
}

///
pub trait Element<CpEvent = NoEvent, CpState = ()>: Debug {
    /// The type of event that
    type Event;

    type AggregateChildrenState: Clone + Default + Debug + PartialEq;
    type BuildOutput: VirtualDom<
        CpEvent,
        CpState,
        Event = Self::Event,
        AggregateChildrenState = Self::AggregateChildrenState,
    >;

    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState);
}

// TODO - Include documentation about what a Virtual DOM is and where the name comes from.
pub trait VirtualDom<CpEvent, CpState>: Debug {
    type AggregateChildrenState: Clone + Default + Debug + PartialEq;
    type TargetWidgetSeq: WidgetSequence;

    type Event;

    // update_value is intended to enable memoize-style HOC
    // where instead of returning a vdom node, it returns
    // something along the lines of struct KeepEverythingAsItWas()
    // Ugh. I'm not explaining this well.
    fn update_value(&mut self, other: Self);

    fn init_tree(&self) -> Self::TargetWidgetSeq;

    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    );

    // TODO - Rename methods
    #[allow(unused_variables)]
    fn process_event(
        &self,
        component_state: &mut CpState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<CpEvent> {
        None
    }

    #[allow(unused_variables)]
    fn process_local_event(
        &self,
        component_state: &mut CpState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoEvent {}

// Used in unit tests
#[allow(dead_code)]
pub(crate) fn assign_empty_state_type(_elem: &impl Element<NoEvent, ()>) {}

#[allow(dead_code)]
pub(crate) fn assign_state_type<CpEvent, CpState, Elem: Element<CpEvent, CpState>>(_elem: &Elem) {}

use crate::elements::{ParentEvent, WithBubbleEvent, WithCallbackEvent, WithMapEvent};

pub trait ElementExt<CpEvent, CpState>: Element<CpEvent, CpState> + Sized {
    fn on<EventParam, Cb: Fn(&mut CpState, EventParam)>(
        self,
        callback: Cb,
    ) -> WithCallbackEvent<CpEvent, CpState, EventParam, Self, Cb>
    where
        Self::Event: ParentEvent<EventParam>,
    {
        WithCallbackEvent {
            element: self,
            callback,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
            _comp_param: Default::default(),
        }
    }

    fn map_event<EventParam, EventReturn, Cb: Fn(&mut CpState, EventParam) -> Option<EventReturn>>(
        self,
        callback: Cb,
    ) -> WithMapEvent<CpEvent, CpState, EventParam, EventReturn, Self, Cb>
    where
        Self::Event: ParentEvent<EventParam>,
        CpEvent: ParentEvent<EventReturn>,
    {
        WithMapEvent {
            element: self,
            callback,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
            _comp_param: Default::default(),
            _comp_return: Default::default(),
        }
    }

    fn bubble_up<Event>(self) -> WithBubbleEvent<CpEvent, CpState, Event, Self>
    where
        Self::Event: ParentEvent<Event>,
        CpEvent: ParentEvent<Event>,
    {
        WithBubbleEvent {
            element: self,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
            _comp_param: Default::default(),
        }
    }
}

impl<CpEvent, CpState, ET: Element<CpEvent, CpState>> ElementExt<CpEvent, CpState> for ET {}
