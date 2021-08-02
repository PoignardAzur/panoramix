use crate::ctx::{ProcessEventCtx, ReconcileCtx};
use crate::glue::GlobalEventCx;
use crate::metadata::Metadata;
use crate::widget_sequence::WidgetSequence;

use std::fmt::Debug;

/// The trait implemented by all GUI elements.
///
/// Every type you use to explicitly create a GUI in Panoramix ([`Button`](crate::elements::Button), [`TextBox`](crate::elements::TextBox), any user-made component) implements Element. You usually don't need to worry about this trait unless you want to implement your own custom element.
///
/// For helper methods that can be called on all elements, see [`ElementExt`].
///
/// ## Note about associated types
///
/// This trait has a lot of associated types; the only one that matters for
/// end users is [`Event`](Self::Event).
pub trait Element: Debug + Clone + 'static {
    /// The type of events this element can raise.
    ///
    /// Events are objects emitted by elements when certain user interactions happen. For
    /// instance, the event type of [`Button`](crate::elements::Button) is [`ButtonClick`](crate::elements::ButtonClick).
    ///
    /// The Event associated type is the type that eg the callback passed to [`ElementExt::on`]
    /// takes as parameter.
    type Event: Debug;

    type ComponentState: Clone + Default + Debug + PartialEq + 'static;
    type AggregateChildrenState: Clone + Default + Debug + PartialEq;
    type BuildOutput: VirtualDom<
        Event = Self::Event,
        AggregateChildrenState = Self::AggregateChildrenState,
    >;

    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState);

    fn get_component_state(_state: &Self::AggregateChildrenState) -> Option<&Self::ComponentState> {
        None
    }
}

// TODO - Include documentation about what a Virtual DOM is and where the name comes from.
pub trait VirtualDom: Debug {
    type Event: Debug;

    type AggregateChildrenState: Clone + Default + Debug + PartialEq;
    type TargetWidgetSeq: WidgetSequence;

    // update_value is intended to enable memoize-style HOC
    // where instead of returning a vdom node, it returns
    // something along the lines of struct KeepEverythingAsItWas()
    // Ugh. I'm not explaining this well.
    fn update_value(&mut self, other: Self)
    where
        Self: Sized,
    {
        *self = other;
    }

    fn init_tree(&self) -> Self::TargetWidgetSeq;

    fn reconcile(
        &self,
        prev_value: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    );

    // TODO - Rename methods
    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        #![allow(unused_variables)]
    }

    fn process_local_event(
        &self,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        #![allow(unused_variables)]
        None
    }
}

use crate::elements::internals::{ParentEvent, WithBubbleEvent, WithCallbackEvent, WithMapEvent};

/// Helper methods that can be called on all elements.
pub trait ElementExt: Element + Sized {
    /// Bind callback to an event.
    ///
    /// When an event is emitted that matches the EventParam type (TODO - see [`ParentEvent`] for
    /// details), call the given callback, with a mutable reference to the component's local state
    /// (see [`CompCtx::get_local_state`](crate::CompCtx::get_local_state)) and the event value.
    fn on<
        EventParam,
        Cb: Fn(&mut ComponentState, EventParam) + Clone,
        ComponentEvent,
        ComponentState,
    >(
        self,
        md: Metadata<ComponentEvent, ComponentState>,
        callback: Cb,
    ) -> WithCallbackEvent<ComponentEvent, ComponentState, EventParam, Self, Cb>
    where
        Self::Event: ParentEvent<EventParam>,
    {
        WithCallbackEvent {
            element: self,
            callback,
            _metadata: md,
            _marker: Default::default(),
        }
    }

    /// Map events from the element to events of the parent component.
    ///
    /// When an event is emitted that matches the EventParam type (TODO - see [`ParentEvent`] for
    /// details), call the given function, with a mutable reference to the component's local state
    /// (see [`CompCtx::get_local_state`](crate::CompCtx::get_local_state)) and the event value.
    ///
    /// If the function returns `Some(...)`, the parent component emits the event.
    fn map_event<
        EventParam,
        EventReturn,
        Cb: Fn(&mut ComponentState, EventParam) -> Option<EventReturn> + Clone,
        ComponentEvent,
        ComponentState,
    >(
        self,
        md: Metadata<ComponentEvent, ComponentState>,
        callback: Cb,
    ) -> WithMapEvent<ComponentEvent, ComponentState, EventParam, EventReturn, Self, Cb>
    where
        Self::Event: ParentEvent<EventParam>,
        ComponentEvent: ParentEvent<EventReturn>,
    {
        WithMapEvent {
            element: self,
            callback,
            _metadata: md,
            _marker: Default::default(),
        }
    }

    /// Passes events from the element to the parent component.
    ///
    /// When an event is emitted that matches the Event type (TODO - see [`ParentEvent`] for
    /// details), the parent component emits the event.
    ///
    /// This is equivalent to a [`map_event`](Self::map_event) where the given callback always
    /// returns `Some(input_event)`.
    fn bubble_up<Event, ComponentEvent, ComponentState>(
        self,
        md: Metadata<ComponentEvent, ComponentState>,
    ) -> WithBubbleEvent<ComponentEvent, ComponentState, Event, Self>
    where
        Self::Event: ParentEvent<Event>,
        ComponentEvent: ParentEvent<Event>,
    {
        WithBubbleEvent {
            element: self,
            _metadata: md,
            _marker: Default::default(),
        }
    }
}

impl<ET: Element> ElementExt for ET {}
