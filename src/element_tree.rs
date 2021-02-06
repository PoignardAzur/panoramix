use crate::glue::{DruidAppData, GlobalEventCx};
use crate::widget_sequence::WidgetSequence;

use druid::{Env, EventCtx};

use std::fmt::Debug;

pub struct ReconcileCtx<'a, 'b, 'c, 'd, 'e> {
    pub event_ctx: &'a mut EventCtx<'d, 'e>,
    pub data: &'b mut DruidAppData,
    pub env: &'c Env,
}

// TODO - must-use
// TODO - Default + Debug bounds
pub trait ElementTree<ComponentState = (), ComponentEvent = NoEvent>: Debug {
    type Event;

    type AggregateChildrenState: Default + Debug;
    type BuildOutput: VirtualDom<
        ComponentState,
        ComponentEvent,
        Event = Self::Event,
        AggregateChildrenState = Self::AggregateChildrenState,
    >;

    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState);
}

// TODO - Include documentation about what a Virtual DOM is and where the name comes from.
pub trait VirtualDom<ComponentState, ComponentEvent>: Debug {
    type AggregateChildrenState: Default + Debug;
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
        component_state: &mut ComponentState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<ComponentEvent> {
        None
    }

    #[allow(unused_variables)]
    fn process_local_event(
        &self,
        component_state: &mut ComponentState,
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
pub(crate) fn assign_empty_state_type(_elem: &impl ElementTree<(), NoEvent>) {}

#[allow(dead_code)]
pub(crate) fn assign_state_type<
    ComponentState,
    ComponentEvent,
    Elem: ElementTree<ComponentState, ComponentEvent>,
>(
    _elem: &Elem,
) {
}

use crate::elements::ParentEvent;
use crate::elements::WithBubbleEvent;
use crate::elements::WithCallbackEvent;
use crate::elements::WithMapEvent;

pub trait ElementTreeExt<ComponentState, ComponentEvent>:
    ElementTree<ComponentState, ComponentEvent> + Sized
{
    fn on<EventParam, Cb: Fn(&mut ComponentState, EventParam)>(
        self,
        callback: Cb,
    ) -> WithCallbackEvent<ComponentState, ComponentEvent, EventParam, Self, Cb>
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

    fn map_event<
        EventParam,
        EventReturn,
        Cb: Fn(&mut ComponentState, EventParam) -> Option<EventReturn>,
    >(
        self,
        callback: Cb,
    ) -> WithMapEvent<ComponentState, ComponentEvent, EventParam, EventReturn, Self, Cb>
    where
        Self::Event: ParentEvent<EventParam>,
        ComponentEvent: ParentEvent<EventReturn>,
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

    fn bubble_up<Event>(self) -> WithBubbleEvent<ComponentState, ComponentEvent, Event, Self>
    where
        Self::Event: ParentEvent<Event>,
        ComponentEvent: ParentEvent<Event>,
    {
        WithBubbleEvent {
            element: self,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
            _comp_param: Default::default(),
        }
    }
}

impl<ComponentState, ComponentEvent, ET: ElementTree<ComponentState, ComponentEvent>>
    ElementTreeExt<ComponentState, ComponentEvent> for ET
{
}
