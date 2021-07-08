use crate::glue::GlobalEventCx;
use crate::widget_sequence::WidgetSequence;

use std::fmt::Debug;

// TODO
use crate::ctx::*;
use crate::metadata::*;

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
/// # use panoramix::{component, CompCtx, Column, Element, ElementExt, Metadata};
/// # use panoramix::elements::{ButtonClick, Button, Label};
/// # type BuyItem = ButtonClick;
/// #
/// #[component]
/// fn StoreItem(ctx: &CompCtx, item_name: String) -> impl Element<BuyItem, u32> {
///     let md = Metadata::<BuyItem, u32>::new();
///     let item_count = ctx.use_local_state::<u32>();
///     Column!(
///         Label::new(format!("Item: {} x{}", item_name, item_count)),
///         Button::new("+")
///             .on_click(md, |item_count, _| {
///                 *item_count += 1;
///             }),
///         Button::new("Buy")
///             .bubble_up::<BuyItem>(md)
///     )
/// }
/// ```
///
/// In this example, because the return type is declared to be `-> impl Element<BuyItem, u32>`, all elements that are returned (Label, Button, Column) will be transitively inferred to implement `Element<BuyItem, u32>`.
///
/// The flip side of this is that constructing an element and not returning it (eg doing `let x = Button::new("...");` and then not using `x`) will lead to a compile error, because the compiler can't infer what `CpEvent` and `CpState` should be.
///
pub trait Element: Debug + Clone {
    /// The type of events this element can raise.
    ///
    /// This is the type that [`ElementExt::on`], [`ElementExt::map_event`] and [`ElementExt::bubble_up`] can take. It's different from the `CpEvent` generic parameter, which is the event the parent component emits.
    ///
    /// In the `StoreItem` example, the `Event` type of buttons is `ButtonClick`, and their `CpEvent` parameter is `BuyItem`.
    type Event;

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
    type Event;

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
        other: &Self,
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

use crate::elements::with_event::{ParentEvent, WithBubbleEvent, WithCallbackEvent, WithMapEvent};

/// Helper methods that can be called on all elements.
pub trait ElementExt: Element + Sized {
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
