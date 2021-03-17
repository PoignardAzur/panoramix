use crate::glue::{DruidAppData, GlobalEventCx};
use crate::widget_sequence::WidgetSequence;

use druid::{Env, EventCtx};
use std::any::Any;
use std::fmt::Debug;

/// Context type passed to all components when building them.
pub struct CompCtx {
    pub(crate) local_state: Box<dyn Any>,
}

impl CompCtx {
    /// Returns the local state of the current component instance.
    ///
    /// Panics if the generic type doesn't match the component's local state type.
    pub fn use_local_state<T: 'static>(&self) -> &T {
        self.local_state.downcast_ref::<T>().unwrap()
    }

    // TODO - add methods
    // use_lifecycle
    // get_vdom_context
}

/// Context required by [`VirtualDom::reconcile`]
pub struct ReconcileCtx<'a, 'b, 'c, 'd, 'e> {
    pub event_ctx: &'a mut EventCtx<'d, 'e>,
    pub data: &'b mut DruidAppData,
    pub env: &'c Env,
}

/// The trait implemented by all GUI elements.
///
/// Every type you use to explicitly create a GUI in Panoramix ([`Button`](crate::elements::Button), [`TextBox`](crate::elements::TextBox), any user-made component) implements Element. You usually don't need to worry about this trait unless you want to implement your own custom element.
///
/// For helper methods that can be called on all elements, see [`ElementExt`].
///
/// ## Note about template resolution
///
/// This trait is parameterized on two template types: `CpEvent` and `CpState`, which represent the event and local-state type of the parent component an element is built in. They are supposed to flow "inwards" with type inference, starting from the `-> impl Element<MyEvent, MyState>` return type of your function.
///
/// To give a concrete example:
///
/// ```rust
/// # use panoramix::{component, CompCtx, Column, Element, ElementExt};
/// # use panoramix::elements::{ButtonClick, Button, Label};
/// # type BuyItem = ButtonClick;
/// #
/// #[component]
/// fn StoreItem(ctx: &CompCtx, item_name: String) -> impl Element<BuyItem, u32> {
///     let item_count = ctx.use_local_state::<u32>();
///     Column!(
///         Label::new(format!("Item: {} x{}", item_name, item_count)),
///         Button::new("+")
///             .on_click(|item_count, _| {
///                 *item_count += 1;
///             }),
///         Button::new("Buy")
///             .bubble_up::<BuyItem>()
///     )
/// }
/// ```
///
/// In this example, because the return type is declared to be `-> impl Element<BuyItem, u32>`, all elements that are returned (Label, Button, Column) will be transitively inferred to implement `Element<BuyItem, u32>`.
///
/// The flip side of this is that constructing an element and not returning it (eg doing `let x = Button::new("...");` and then not using `x`) will lead to a compile error, because the compiler can't infer what `CpEvent` and `CpState` should be.
///
pub trait Element<CpEvent = NoEvent, CpState = ()>: Debug {
    /// The type of events this element can raise.
    ///
    /// This is the type that [`ElementExt::on`], [`ElementExt::map_event`] and [`ElementExt::bubble_up`] can take. It's different from the `CpEvent` generic parameter, which is the event the parent component emits.
    ///
    /// In the `StoreItem` example, the `Event` type of buttons is `ButtonClick`, and their `CpEvent` parameter is `BuyItem`.
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
    fn update_value(&mut self, other: Self)
    where
        Self: Sized,
    {
        *self = other;
    }

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

/// Placeholder type for elements that don't raise events.
///
/// Equivalent to `!`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoEvent {}

// Used in unit tests
#[allow(dead_code)]
pub(crate) fn assign_empty_state_type(_elem: &impl Element<NoEvent, ()>) {}

#[allow(dead_code)]
pub(crate) fn assign_state_type<CpEvent, CpState, Elem: Element<CpEvent, CpState>>(_elem: &Elem) {}

use crate::elements::with_event::{ParentEvent, WithBubbleEvent, WithCallbackEvent, WithMapEvent};

/// Helper methods that can be called on all elements.
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
