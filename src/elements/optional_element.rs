use crate::ctx::{ProcessEventCtx, ReconcileCtx};
use crate::element_tree::{Element, VirtualDom};
use crate::glue::GlobalEventCx;
use crate::metadata::{NoEvent, NoState};

use either::{Either, Left, Right};
use tracing::{debug_span, info, instrument};
use tracing_unwrap::OptionExt;

impl<Child: Element> Element for Option<Child> {
    type Event = NoEvent;
    type ComponentState = NoState;
    type AggregateChildrenState = Option<Child::AggregateChildrenState>;
    type BuildOutput = Option<Child::BuildOutput>;

    #[instrument(name = "Option", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        if let Some(child) = self {
            let (output, state) = child.build(prev_state.unwrap_or_default());
            (Some(output), Some(state))
        } else {
            (None, None)
        }
    }
}

impl<Child: VirtualDom> VirtualDom for Option<Child> {
    type Event = NoEvent;
    type AggregateChildrenState = Option<Child::AggregateChildrenState>;
    type TargetWidgetSeq = Option<Child::TargetWidgetSeq>;

    #[instrument(name = "Option", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        if let Some(child) = self {
            Some(child.init_tree())
        } else {
            None
        }
    }

    #[instrument(name = "Option", skip(self, other, widget_seq, ctx))]
    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        match (self, other) {
            (Some(child), Some(other_child)) => {
                child.reconcile(other_child, &mut widget_seq.as_mut().unwrap_or_log(), ctx);
            }
            (Some(child), None) => {
                debug_span!("init_tree").in_scope(|| {
                    info!("creating child");
                    *widget_seq = Some(child.init_tree());
                });
            }
            (None, Some(_other_child)) => {
                info!("removing child");
                *widget_seq = None;
            }
            (None, None) => {}
        }
    }

    #[instrument(name = "Option", skip(self, comp_ctx, children_state, widget_seq, cx))]
    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        if let Some(child) = self.as_ref() {
            child.process_event(
                comp_ctx,
                children_state.as_mut().unwrap_or_log(),
                widget_seq.as_mut().unwrap_or_log(),
                cx,
            );
        }
    }
}

// ----

impl<ChildLeft: Element, ChildRight: Element> Element for Either<ChildLeft, ChildRight> {
    type Event = NoEvent;
    type ComponentState = NoState;
    type AggregateChildrenState =
        Option<Either<ChildLeft::AggregateChildrenState, ChildRight::AggregateChildrenState>>;
    type BuildOutput = Either<ChildLeft::BuildOutput, ChildRight::BuildOutput>;

    #[instrument(name = "Either", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        match self {
            Left(child) => {
                let prev_state = prev_state.map_or(None, |ps| ps.left()).unwrap_or_default();
                let (output, state) = child.build(prev_state);
                (Left(output), Some(Left(state)))
            }
            Right(child) => {
                let prev_state = prev_state.map_or(None, |ps| ps.right()).unwrap_or_default();
                let (output, state) = child.build(prev_state);
                (Right(output), Some(Right(state)))
            }
        }
    }
}

impl<ChildLeft: VirtualDom, ChildRight: VirtualDom> VirtualDom for Either<ChildLeft, ChildRight> {
    type Event = NoEvent;
    type AggregateChildrenState =
        Option<Either<ChildLeft::AggregateChildrenState, ChildRight::AggregateChildrenState>>;
    type TargetWidgetSeq = Either<ChildLeft::TargetWidgetSeq, ChildRight::TargetWidgetSeq>;

    #[instrument(name = "Either", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        match self {
            Left(child) => Left(child.init_tree()),
            Right(child) => Right(child.init_tree()),
        }
    }

    #[instrument(name = "Either", skip(self, other, widget_seq, ctx))]
    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        match (self, &other) {
            (Left(child), Left(other)) => {
                // TODO - Add more detailed log
                let widget_seq = &mut widget_seq.as_mut().left().expect_or_log("The previous value of this element was Left. Expected Left widget.");
                child.reconcile(other, widget_seq, ctx);
            }
            (Right(child), Right(other)) => {
                // TODO - Add more detailed log
                let widget_seq = &mut widget_seq.as_mut().right().expect_or_log("The previous value of this element was Right. Expected Right widget.");
                child.reconcile(other, widget_seq, ctx);
            }

            (Left(child), Right(_other)) => {
                debug_span!("init_tree").in_scope(|| {
                    info!("creating child");
                    *widget_seq = Left(child.init_tree());
                });
            }
            (Right(child), Left(_other)) => {
                debug_span!("init_tree").in_scope(|| {
                    info!("creating child");
                    *widget_seq = Right(child.init_tree());
                });
            }
        }
    }

    #[instrument(name = "Either", skip(self, comp_ctx, children_state, widget_seq, cx))]
    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        match self {
            Left(child) => child.process_event(
                comp_ctx,
                &mut children_state
                    .as_mut()
                    .unwrap_or_log()
                    .as_mut()
                    .left()
                    .unwrap_or_log(),
                widget_seq.as_mut().left().unwrap_or_log(),
                cx,
            ),
            Right(child) => child.process_event(
                comp_ctx,
                &mut children_state
                    .as_mut()
                    .unwrap_or_log()
                    .as_mut()
                    .right()
                    .unwrap_or_log(),
                widget_seq.as_mut().right().unwrap_or_log(),
                cx,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::button::{Button, ButtonData};
    use crate::elements::label::{Label, LabelData};
    use crate::flex::FlexParams;
    use crate::test_harness::Harness;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn new_option() {
        let mut option_label = Some(Label::new("Hello"));
        let (option_label_data, _) = option_label.clone().build(None);

        assert_debug_snapshot!(option_label);
        assert_debug_snapshot!(option_label_data);
        assert_eq!(option_label_data, Some(LabelData::new("Hello")));

        option_label = None;
        let (option_label_data, _) = option_label.clone().build(None);

        assert_debug_snapshot!(option_label);
        assert_debug_snapshot!(option_label_data);
        assert_eq!(option_label_data, None);
    }

    #[test]
    fn new_either() {
        let mut either_elem = Left(Label::new("Hello"));
        let (either_elem_data, _) = either_elem.clone().build(None);

        assert_debug_snapshot!(either_elem);
        assert_debug_snapshot!(either_elem_data);
        assert_eq!(either_elem_data, Left(LabelData::new("Hello")));

        let button_data = ButtonData {
            text: String::from("World"),
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
            ..Default::default()
        };

        either_elem = Right(Button::new("World"));
        let (either_elem_data, _) = either_elem.clone().build(None);

        assert_debug_snapshot!(either_elem);
        assert_debug_snapshot!(either_elem_data);
        assert_eq!(either_elem_data, Right(button_data));
    }

    #[test]
    fn test_option_widget() {
        let some_label = Some(Label::new("Hello"));

        Harness::run_test_window(some_label, |harness| {
            let root_state = harness.get_root_debug_state();
            assert_debug_snapshot!(root_state);

            let new_label = Some(Label::new("World"));
            harness.update_root_element(new_label);

            let root_state_2 = harness.get_root_debug_state();
            assert_debug_snapshot!(root_state_2);

            harness.update_root_element(None);

            let root_state_3 = harness.get_root_debug_state();
            assert_debug_snapshot!(root_state_3);

            let other_label = Some(Label::new("Goodbye"));
            harness.update_root_element(other_label);

            let root_state_4 = harness.get_root_debug_state();
            assert_debug_snapshot!(root_state_4);
        });
    }

    #[test]
    fn test_either_widget() {
        type LabelButton = Either<Label, Button>;
        let label: LabelButton = Left(Label::new("Hello"));

        Harness::run_test_window(label, |harness| {
            let root_state = harness.get_root_debug_state();
            assert_debug_snapshot!(root_state);

            let new_label: LabelButton = Left(Label::new("World"));
            harness.update_root_element(new_label);

            let root_state_2 = harness.get_root_debug_state();
            assert_debug_snapshot!(root_state_2);

            let button: LabelButton = Right(Button::new("ThisIsAButton"));
            harness.update_root_element(button);

            let root_state_3 = harness.get_root_debug_state();
            assert_debug_snapshot!(root_state_3);

            // TODO - Test reconcile button
            // let new_button: LabelButton = Right(Button::new("AnotherButton"));
            // harness.update_root_element(new_button);
            // let root_state_4 = harness.get_root_debug_state();
            // assert_debug_snapshot!(root_state_4);

            let final_label: LabelButton = Left(Label::new("Goodbye"));
            harness.update_root_element(final_label);

            let root_state_5 = harness.get_root_debug_state();
            assert_debug_snapshot!(root_state_5);
        });
    }
}
