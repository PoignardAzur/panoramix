use crate::ctx::{ProcessEventCtx, ReconcileCtx};
use crate::element_tree::{Element, VirtualDom};
use crate::metadata::{NoEvent, NoState};

use crate::glue::GlobalEventCx;

use derivative::Derivative;
use std::sync::mpsc::Sender;
use tracing::instrument;

/// Log events of child element. For unit tests only.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct EventLogger<Child: Element> {
    pub child: Child,
    /// Queue to which events of `self.child` are sent.
    pub event_queue: Sender<Child::Event>,
}

#[derive(Derivative)]
#[derivative(Clone(bound = "Child: Clone"), Debug(bound = ""))]
pub struct EventLoggerData<Child: VirtualDom> {
    pub child: Child,
    pub event_queue: Sender<Child::Event>,
}

// ----

impl<Child: Element> EventLogger<Child> {
    pub fn new(event_queue: Sender<Child::Event>, child: Child) -> Self {
        EventLogger { child, event_queue }
    }
}

// ----

impl<Child: Element> Element for EventLogger<Child> {
    type Event = NoEvent;

    type ComponentState = NoState;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = EventLoggerData<Child::BuildOutput>;

    #[instrument(name = "EventLogger", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (element, child_state) = self.child.build(prev_state);
        (
            EventLoggerData {
                child: element,
                event_queue: self.event_queue,
            },
            child_state,
        )
    }
}

impl<Child: VirtualDom> VirtualDom for EventLoggerData<Child> {
    type Event = NoEvent;

    type AggregateChildrenState = Child::AggregateChildrenState;
    type TargetWidgetSeq = Child::TargetWidgetSeq;

    #[instrument(name = "EventLogger", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        self.child.init_tree()
    }

    #[instrument(name = "EventLogger", skip(self, prev_value, widget_seq, ctx))]
    fn reconcile(
        &self,
        prev_value: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.child.reconcile(&prev_value.child, widget_seq, ctx);
    }

    #[instrument(
        name = "EventLogger",
        skip(self, comp_ctx, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
        children_state: &mut Child::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        self.child
            .process_event(comp_ctx, children_state, widget_seq, cx);

        let local_event = self
            .child
            .process_local_event(children_state, widget_seq, cx);
        if let Some(local_event) = local_event {
            let _ = self.event_queue.send(local_event);
        }
    }
}
