use crate::glue::{DruidAppData, GlobalEventCx};
use crate::widget_sequence::WidgetSequence;

use derivative::Derivative;
use druid::{Env, EventCtx};
use std::any::{type_name, Any, TypeId};
use std::fmt::Debug;

/// Context type passed to all components when building them.
pub struct CompCtx<'a> {
    pub(crate) local_state: &'a dyn Any,
}

impl<'a> CompCtx<'a> {
    /// Returns the local state of the current component instance.
    ///
    /// Panics if the generic type doesn't match the component's local state type.
    pub fn use_local_state<T: 'static>(&self) -> &'a T {
        if (*self.local_state).type_id() == TypeId::of::<NoState>() {
            panic!("error: 'use_local_state' cannot be called for a component whose root element isn't ComponentOutput")
        }
        self.local_state.downcast_ref::<T>().unwrap()
    }

    // TODO - add methods
    // get metadata
    // use_lifecycle
    // get_vdom_context
}

/// Context required by [`VirtualDom::reconcile`]
pub struct ReconcileCtx<'a, 'b, 'c, 'd, 'e> {
    pub event_ctx: &'a mut EventCtx<'d, 'e>,
    pub data: &'b mut DruidAppData,
    pub env: &'c Env,
}

pub struct ProcessEventCtx<'e, 's> {
    pub event_queue: &'e mut dyn Any,
    pub state: &'s mut dyn Any,
}

impl<'e, 's> ProcessEventCtx<'e, 's> {
    pub fn event_queue<ComponentEvent: 'static, ComponentState: 'static>(
        &mut self,
        md: Metadata<ComponentEvent, ComponentState>,
    ) -> &mut Vec<ComponentEvent> {
        #![allow(unused_variables)]
        let type_id = (*self.event_queue).type_id();
        self.event_queue
            .downcast_mut::<Vec<ComponentEvent>>()
            .expect(&format!(
                "internal type error: event handler expected {:?} ({}), parent component gave {:?}",
                TypeId::of::<Vec<ComponentEvent>>(),
                type_name::<Vec<ComponentEvent>>(),
                type_id,
            ))
    }

    pub fn state<ComponentEvent: 'static, ComponentState: 'static>(
        &mut self,
        md: Metadata<ComponentEvent, ComponentState>,
    ) -> &mut ComponentState {
        #![allow(unused_variables)]
        let type_id = (*self.event_queue).type_id();
        // TODO - Add one failing test for this
        self.state.downcast_mut::<ComponentState>().expect(&format!(
            "internal type error: event handler expected {:?} ({}), parent component gave {:?}",
            TypeId::of::<ComponentState>(),
            type_name::<ComponentState>(),
            type_id,
        ))
    }
}

#[derive(Derivative)]
#[derivative(
    Clone(bound = ""),
    Copy(bound = ""),
    Default(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
pub struct Metadata<ComponentEvent, ComponentState> {
    _marker: std::marker::PhantomData<(ComponentEvent, ComponentState)>,
}

impl<ComponentEvent, ComponentState> Metadata<ComponentEvent, ComponentState> {
    pub fn new() -> Self {
        Default::default()
    }
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

/// Placeholder type for elements that don't raise events.
///
/// Equivalent to `!`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoEvent {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoState;

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
