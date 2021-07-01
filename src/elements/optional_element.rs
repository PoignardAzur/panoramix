use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;

use crate::element_tree::ReconcileCtx;

use either::{Either, Left, Right};
use tracing::{debug_span, info, instrument};
use tracing_unwrap::OptionExt;

impl<CpEvent, CpState, Child: Element<CpEvent, CpState>> Element<CpEvent, CpState>
    for Option<Child>
{
    type Event = NoEvent;
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

impl<CpEvent, CpState, Child: VirtualDom<CpEvent, CpState>> VirtualDom<CpEvent, CpState>
    for Option<Child>
{
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
        if let Some(child) = self.as_ref() {
            if let Some(other_child) = other {
                child.reconcile(other_child, &mut widget_seq.as_mut().unwrap_or_log(), ctx);
            } else {
                debug_span!("init_tree").in_scope(|| {
                    info!("creating child");
                    *widget_seq = Some(child.init_tree());
                });
            }
        }
    }

    #[instrument(
        name = "Option",
        skip(self, component_state, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        component_state: &mut CpState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<CpEvent> {
        let child = self.as_ref()?;
        child.process_event(
            component_state,
            children_state.as_mut().unwrap_or_log(),
            widget_seq.as_mut().unwrap_or_log(),
            cx,
        )
    }
}

// ----

impl<
        CpState,
        CpEvent,
        ChildLeft: Element<CpEvent, CpState>,
        ChildRight: Element<CpEvent, CpState>,
    > Element<CpEvent, CpState> for Either<ChildLeft, ChildRight>
{
    type Event = NoEvent;
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

impl<
        CpState,
        CpEvent,
        ChildLeft: VirtualDom<CpEvent, CpState>,
        ChildRight: VirtualDom<CpEvent, CpState>,
    > VirtualDom<CpEvent, CpState> for Either<ChildLeft, ChildRight>
{
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
        match self {
            Left(child) => {
                if let Right(_) = &other {
                    debug_span!("init_tree").in_scope(|| {
                        info!("creating child");
                        *widget_seq = Left(child.init_tree());
                    });
                }
                child.reconcile(
                    other.as_ref().left().unwrap_or_log(),
                    &mut widget_seq.as_mut().left().unwrap_or_log(),
                    ctx,
                );
            }
            Right(child) => {
                if let Left(_) = &other {
                    debug_span!("init_tree").in_scope(|| {
                        info!("creating child");
                        *widget_seq = Right(child.init_tree());
                    });
                }
                child.reconcile(
                    other.as_ref().right().unwrap_or_log(),
                    &mut widget_seq.as_mut().right().unwrap_or_log(),
                    ctx,
                );
            }
        }
    }

    #[instrument(
        name = "Either",
        skip(self, component_state, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        component_state: &mut CpState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<CpEvent> {
        match self {
            Left(child) => child.process_event(
                component_state,
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
                component_state,
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
    use crate::element_tree::assign_empty_state_type;
    use crate::elements::button::{Button, ButtonData};
    use crate::elements::label::{Label, LabelData};
    use crate::flex::FlexParams;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn new_option() {
        let mut option_label = Some(Label::new("Hello"));
        let (option_label_data, _) = option_label.clone().build(None);

        assert_debug_snapshot!(option_label);
        assert_debug_snapshot!(option_label_data);
        assert_eq!(option_label_data, Some(LabelData::new("Hello")));

        assign_empty_state_type(&option_label);

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

        assign_empty_state_type(&either_elem);
    }

    // TODO - Widget tests

}
