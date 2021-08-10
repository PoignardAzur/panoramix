use crate::ctx::ReconcileCtx;
use crate::element_tree::{Element, ElementExt, VirtualDom};
use crate::glue::{Action, GlobalEventCx, WidgetId};
use crate::metadata::{Metadata, NoState};
use crate::widgets::ClickableWidget;

use tracing::{instrument, trace};

// TODO - doc
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Clickable<Child: Element> {
    pub child: Child,
    pub reserved_widget_id: Option<WidgetId>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClickableData<Child: VirtualDom> {
    pub child: Child,
    pub reserved_widget_id: Option<WidgetId>,
}

/// Event emitted when a [Clickable] is clicked.
///
/// Note: Might hold data like "mouse position" or "widget id" future versions.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClickEvent;

//
// --- IMPLS

impl<Child: Element> Clickable<Child> {
    pub fn new(child: Child) -> Self {
        Clickable {
            child,
            reserved_widget_id: None,
        }
    }

    /// For unit tests only.
    ///
    /// The widget created by this element always has the same id. If two widgets are created
    /// with the same id (for instance, because the same widget is returned twice because of
    /// a copy-paste error), impredictable behavior may ensue.
    pub fn with_reserved_id(self, widget_id: WidgetId) -> Self {
        Clickable {
            reserved_widget_id: Some(widget_id),
            ..self
        }
    }

    /// Provide a closure to be called when the child widget is clicked.
    pub fn on_click<ComponentEvent: 'static, ComponentState: 'static>(
        self,
        md: Metadata<ComponentEvent, ComponentState>,
        callback: impl Fn(&mut ComponentState, ClickEvent) + Clone + 'static,
    ) -> impl Element {
        self.on(md, callback)
    }
}

impl<Child: Element> Element for Clickable<Child> {
    type Event = ClickEvent;

    type ComponentState = NoState;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = ClickableData<Child::BuildOutput>;

    #[instrument(name = "Clickable", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (
        ClickableData<Child::BuildOutput>,
        Self::AggregateChildrenState,
    ) {
        let (element, child_state) = self.child.build(prev_state);
        (
            ClickableData {
                child: element,
                reserved_widget_id: self.reserved_widget_id,
            },
            child_state,
        )
    }
}

impl<Child: VirtualDom> VirtualDom for ClickableData<Child> {
    type Event = ClickEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type TargetWidgetSeq = ClickableWidget<Child::TargetWidgetSeq>;

    #[instrument(name = "Clickable", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        let id = self.reserved_widget_id.unwrap_or_else(WidgetId::next);
        ClickableWidget::new(self.child.init_tree(), id)
    }

    #[instrument(name = "Clickable", skip(self, prev_value, widget_seq, ctx))]
    fn reconcile(
        &self,
        prev_value: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.child
            .reconcile(&prev_value.child, widget_seq.children_mut(), ctx);
    }

    #[instrument(name = "Clickable", skip(self, _children_state, widget, cx))]
    fn process_local_event(
        &self,
        _children_state: &mut Self::AggregateChildrenState,
        widget: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<ClickEvent> {
        // FIXME - Rework event dispatching
        let id = widget.id();
        if let Some(Action::Clicked) = cx.app_data.dequeue_action(id) {
            trace!("Processed click");
            Some(ClickEvent)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::label::Label;
    use crate::test_harness::Harness;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn new_clickable() {
        let clickable = Clickable::new(Label::new("Hello"));

        let (clickable_data, ()) = clickable.clone().build(());

        assert_debug_snapshot!(clickable);
        assert_debug_snapshot!(clickable_data);
    }

    #[test]
    fn clickable_widget() {
        let clickable = Clickable::new(Label::new("Hello"));

        Harness::run_test_window(clickable, |_harness| {
            // FIXME
            //let clickable_state = harness.get_root_debug_state();
            //assert_debug_snapshot!(clickable_state);

            // FIXME - Test reconcile() method with child
        });
    }

    #[test]
    fn clickable_press() {
        use crate::elements::event_logger::EventLogger;
        use std::sync::mpsc::channel;

        let (event_sender, event_receiver) = channel();
        let clickable_id = WidgetId::reserved(1);
        let clickable = EventLogger::new(
            event_sender,
            Clickable::new(Label::new("Hello")).with_reserved_id(clickable_id),
        );

        Harness::run_test_window(clickable, |harness| {
            // FIXME
            // assert_debug_snapshot!(harness.get_root_debug_state());

            harness.mouse_click_on(clickable_id);

            let click_event = event_receiver.try_recv();
            assert_debug_snapshot!(click_event);
        });
    }
}
