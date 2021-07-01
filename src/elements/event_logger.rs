// FIXME - Element used in unit tests to record events emitted by a child event

use crate::element_tree::ReconcileCtx;
use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;

use derivative::Derivative;
use std::sync::mpsc::Sender;
use tracing::instrument;

// PartialEq?
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct EventLogger<Child: Element<CpEvent, CpState>, CpEvent = NoEvent, CpState = ()> {
    pub child: Child,
    pub event_queue: Sender<Child::Event>,
    pub _comp_state: std::marker::PhantomData<CpState>,
    pub _comp_event: std::marker::PhantomData<CpEvent>,
}

// PartialEq?
#[derive(Derivative)]
#[derivative(Clone(bound = "Child: Clone"), Debug(bound = ""))]
pub struct EventLoggerData<Child: VirtualDom<CpEvent, CpState>, CpEvent = NoEvent, CpState = ()> {
    pub child: Child,
    pub event_queue: Sender<Child::Event>,
    pub _comp_state: std::marker::PhantomData<CpState>,
    pub _comp_event: std::marker::PhantomData<CpEvent>,
}

// ----

impl<CpEvent, CpState, Child: Element<CpEvent, CpState>> EventLogger<Child, CpEvent, CpState> {
    pub fn new(event_queue: Sender<Child::Event>, child: Child) -> Self {
        EventLogger {
            child,
            event_queue,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
        }
    }
}

// ----

impl<CpEvent, CpState, Child: Element<CpEvent, CpState>> Element<CpEvent, CpState>
    for EventLogger<Child, CpEvent, CpState>
{
    type Event = NoEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = EventLoggerData<Child::BuildOutput, CpEvent, CpState>;

    #[instrument(name = "EventLogger", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (element, component_state) = self.child.build(prev_state);
        (
            EventLoggerData {
                child: element,
                event_queue: self.event_queue,
                _comp_state: Default::default(),
                _comp_event: Default::default(),
            },
            component_state,
        )
    }
}

impl<CpEvent, CpState, Child: VirtualDom<CpEvent, CpState>> VirtualDom<CpEvent, CpState>
    for EventLoggerData<Child, CpEvent, CpState>
{
    type Event = NoEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;

    type TargetWidgetSeq = Child::TargetWidgetSeq;

    #[instrument(name = "EventLogger", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        self.child.init_tree()
    }

    #[instrument(name = "EventLogger", skip(self, other, widget_seq, ctx))]
    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.child.reconcile(&other.child, widget_seq, ctx);
    }

    #[instrument(
        name = "EventLogger",
        skip(self, component_state, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        component_state: &mut CpState,
        children_state: &mut Child::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<CpEvent> {
        let local_event =
            self.child
                .process_local_event(component_state, children_state, widget_seq, cx);
        if let Some(local_event) = local_event {
            let _ = self.event_queue.send(local_event);
        }
        None
    }

    // FIXME
    #[instrument(
        name = "EventLogger",
        skip(self, component_state, children_state, widget_seq, cx)
    )]
    fn process_local_event(
        &self,
        component_state: &mut CpState,
        children_state: &mut Child::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<NoEvent> {
        self.process_event(component_state, children_state, widget_seq, cx);
        None
    }
}
