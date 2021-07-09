use crate::ctx::{ProcessEventCtx, ReconcileCtx};
use crate::element_tree::{Element, VirtualDom};
use crate::glue::GlobalEventCx;
use crate::metadata::NoState;
use crate::widgets::WidgetSeqBox;

use std::any::Any;
use std::fmt::Debug;

// --- STATE ---

pub trait AnyState: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn print_type(&self) {
        println!("{:#?}", std::any::type_name::<Self>());
    }

    fn dyn_clone(&self) -> Box<dyn AnyState>;
    fn dyn_eq(&self, other: &Box<dyn AnyState>) -> bool;
}

impl<T> AnyState for T
where
    T: Clone + Default + Debug + PartialEq + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn AnyState> {
        Box::new(self.clone())
    }

    fn dyn_eq(&self, other: &Box<dyn AnyState>) -> bool {
        if let Some(other) = other.as_ref().as_any().downcast_ref::<Self>() {
            other.eq(self)
        } else {
            false
        }
    }
}

pub struct AnyStateBox {
    value: Box<dyn AnyState>,
}

impl AnyStateBox {
    pub fn new(state: impl AnyState) -> Self {
        AnyStateBox {
            value: Box::new(state),
        }
    }
}

impl Clone for AnyStateBox {
    fn clone(&self) -> Self {
        AnyStateBox {
            value: self.value.dyn_clone(),
        }
    }
}

impl Debug for AnyStateBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.value.fmt(f)
    }
}

impl PartialEq for AnyStateBox {
    fn eq(&self, other: &Self) -> bool {
        self.value.dyn_eq(&other.value)
    }
}

// --- ELEMENT ---

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct ErasedElement<Child: Element> {
    child: Option<Child>,
}

impl<Child: Element> Debug for ErasedElement<Child> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.child.as_ref().unwrap().fmt(f)
    }
}

trait AnyElement: Any + Debug {
    type Event;

    fn print_type(&self) {
        println!("{:#?}", std::any::type_name::<Self>());
    }

    fn dyn_clone(&self) -> Box<dyn AnyElement<Event = Self::Event>>;

    fn build(
        &mut self,
        prev_state: Option<AnyStateBox>,
    ) -> (
        Box<dyn AnyVirtualDom<Event = Self::Event>>,
        Option<AnyStateBox>,
    );
}

impl<Child: Element + 'static> AnyElement for ErasedElement<Child> {
    type Event = Child::Event;

    fn dyn_clone(&self) -> Box<dyn AnyElement<Event = Self::Event>> {
        Box::new(self.clone())
    }

    fn build(
        &mut self,
        prev_state: Option<AnyStateBox>,
    ) -> (
        Box<dyn AnyVirtualDom<Event = Self::Event>>,
        Option<AnyStateBox>,
    ) {
        let child = self.child.take().unwrap();

        let prev_state = if let Some(mut prev_state) = prev_state {
            std::mem::take(
                prev_state
                    .value
                    .as_mut_any()
                    .downcast_mut::<Child::AggregateChildrenState>()
                    .unwrap(),
            )
        } else {
            Default::default()
        };

        let (output, state) = child.build(prev_state);

        (
            Box::new(ErasedVirtualDom { child: output }),
            Some(AnyStateBox {
                value: Box::new(state),
            }),
        )
    }
}

// -

pub struct ElementBox<Event> {
    child: Box<dyn AnyElement<Event = Event>>,
}

impl<Event> ElementBox<Event> {
    pub fn new(child: impl Element<Event = Event> + 'static) -> Self {
        ElementBox {
            child: Box::new(ErasedElement { child: Some(child) }),
        }
    }
}

impl<Event> Debug for ElementBox<Event> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.child.fmt(f)
    }
}

impl<Event> Clone for ElementBox<Event> {
    fn clone(&self) -> Self {
        ElementBox {
            child: self.child.dyn_clone(),
        }
    }
}

impl<Event> Element for ElementBox<Event> {
    type Event = Event;
    type ComponentState = NoState;
    type AggregateChildrenState = Option<AnyStateBox>;
    type BuildOutput = VirtualDomBox<Event>;

    fn build(self, prev_state: Option<AnyStateBox>) -> (VirtualDomBox<Event>, Option<AnyStateBox>) {
        let mut child = self.child;
        let (output, state) = child.build(prev_state);

        (VirtualDomBox { child: output }, state)
    }
}

// --- VIRTUAL_DOM ---

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct ErasedVirtualDom<Child: VirtualDom> {
    child: Child,
}

impl<Child: VirtualDom> Debug for ErasedVirtualDom<Child> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.child.fmt(f)
    }
}

pub trait AnyVirtualDom: Any + Debug {
    type Event;

    fn as_any(&self) -> &dyn Any;

    fn print_type(&self) {
        println!("{:#?}", std::any::type_name::<Self>());
    }

    fn init_tree(&self) -> WidgetSeqBox;

    fn reconcile(
        &self,
        other: &Box<dyn AnyVirtualDom<Event = Self::Event>>,
        widget_seq: &mut WidgetSeqBox,
        ctx: &mut ReconcileCtx,
    );

    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
        children_state: &mut Option<AnyStateBox>,
        widget_seq: &mut WidgetSeqBox,
        cx: &mut GlobalEventCx,
    );

    fn process_local_event(
        &self,
        children_state: &mut Option<AnyStateBox>,
        widget_seq: &mut WidgetSeqBox,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event>;
}

impl<Child: VirtualDom + 'static> AnyVirtualDom for ErasedVirtualDom<Child> {
    type Event = Child::Event;

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn init_tree(&self) -> WidgetSeqBox {
        WidgetSeqBox {
            value: Box::new(self.child.init_tree()),
        }
    }

    fn reconcile(
        &self,
        other: &Box<dyn AnyVirtualDom<Event = Self::Event>>,
        widget_seq: &mut WidgetSeqBox,
        ctx: &mut ReconcileCtx,
    ) {
        let other = other.as_any().downcast_ref::<Self>().unwrap();
        let widget_seq = widget_seq
            .value
            .as_mut_any()
            .downcast_mut::<Child::TargetWidgetSeq>()
            .unwrap();
        self.child.reconcile(&other.child, widget_seq, ctx);
    }

    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
        children_state: &mut Option<AnyStateBox>,
        widget_seq: &mut WidgetSeqBox,
        cx: &mut GlobalEventCx,
    ) {
        let children_state = children_state
            .as_mut()
            .unwrap()
            .value
            .as_mut_any()
            .downcast_mut::<Child::AggregateChildrenState>()
            .unwrap();
        let widget_seq = widget_seq
            .value
            .as_mut_any()
            .downcast_mut::<Child::TargetWidgetSeq>()
            .unwrap();
        self.child
            .process_event(comp_ctx, children_state, widget_seq, cx)
    }

    fn process_local_event(
        &self,
        children_state: &mut Option<AnyStateBox>,
        widget_seq: &mut WidgetSeqBox,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        let children_state = children_state
            .as_mut()
            .unwrap()
            .value
            .as_mut_any()
            .downcast_mut::<Child::AggregateChildrenState>()
            .unwrap();
        let widget_seq = widget_seq
            .value
            .as_mut_any()
            .downcast_mut::<Child::TargetWidgetSeq>()
            .unwrap();
        self.child
            .process_local_event(children_state, widget_seq, cx)
    }
}

// -

pub struct VirtualDomBox<Event> {
    child: Box<dyn AnyVirtualDom<Event = Event>>,
}

impl<Event> VirtualDomBox<Event> {
    pub fn new(child: impl VirtualDom<Event = Event> + 'static) -> Self {
        VirtualDomBox {
            child: Box::new(ErasedVirtualDom { child: child }),
        }
    }
}

impl<Event> Debug for VirtualDomBox<Event> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.child.fmt(f)
    }
}

impl<Event> VirtualDom for VirtualDomBox<Event> {
    // TODO - Accept any Event parent of Self::Event
    type Event = Event;
    type AggregateChildrenState = Option<AnyStateBox>;
    type TargetWidgetSeq = WidgetSeqBox;

    fn init_tree(&self) -> Self::TargetWidgetSeq {
        self.child.init_tree()
    }

    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.child.reconcile(&other.child, widget_seq, ctx);
    }

    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
        children_state: &mut Option<AnyStateBox>,
        widget_seq: &mut WidgetSeqBox,
        cx: &mut GlobalEventCx,
    ) {
        self.child
            .process_event(comp_ctx, children_state, widget_seq, cx)
    }

    fn process_local_event(
        &self,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        self.child
            .process_local_event(children_state, widget_seq, cx)
    }
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::{Button, Label};
    use crate::test_harness::Harness;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn any_state() {
        let state1 = AnyStateBox::new(42);
        let state2 = state1.clone();

        assert!(state1 == state2);
    }

    #[test]
    fn new_element() {
        let label = ElementBox::new(Label::new("Hello"));
        assert_debug_snapshot!(label);

        let (label_data, _state) = label.build(None);
        assert_debug_snapshot!(label_data);

        // TODO - check state
    }

    #[test]
    fn boxed_label_widget() {
        let label = ElementBox::new(Label::new("Hello"));

        Harness::run_test_window(label, |harness| {
            let label_state = harness.get_root_debug_state();
            assert_debug_snapshot!(label_state);

            let new_label = ElementBox::new(Label::new("World"));
            harness.update_root_element(new_label);

            let label_state_2 = harness.get_root_debug_state();
            assert_debug_snapshot!(label_state_2);
        });
    }

    // FIXME - Test doesn't use box
    #[test]
    fn boxed_button_press() {
        use crate::elements::event_logger::EventLogger;
        use crate::glue::WidgetId;
        use std::sync::mpsc::channel;

        let (event_sender, event_receiver) = channel();
        let button_id = WidgetId::reserved(1);
        let button = EventLogger::new(
            event_sender,
            Button::new("Hello").with_reserved_id(button_id),
        );

        Harness::run_test_window(button, |harness| {
            assert_debug_snapshot!(harness.get_root_debug_state());

            harness.mouse_click_on(button_id);

            let click_event = event_receiver.try_recv();
            assert_debug_snapshot!(click_event);
        });
    }
}
