use crate::element_tree::{ElementTree, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;

use either::{Either, Left, Right};
use tracing::instrument;
use tracing_unwrap::OptionExt;

impl<ComponentState, ComponentEvent, Child: ElementTree<ComponentState, ComponentEvent>>
    ElementTree<ComponentState, ComponentEvent> for Option<Child>
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

impl<ComponentState, ComponentEvent, Child: VirtualDom<ComponentState, ComponentEvent>>
    VirtualDom<ComponentState, ComponentEvent> for Option<Child>
{
    type Event = NoEvent;
    type AggregateChildrenState = Option<Child::AggregateChildrenState>;
    type TargetWidgetSeq = Option<Child::TargetWidgetSeq>;

    #[instrument(name = "Option", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Option", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        if let Some(child) = self {
            Some(child.init_tree())
        } else {
            None
        }
    }

    #[instrument(name = "Option", skip(self, other, widget_seq))]
    fn reconcile(&self, other: &Self, widget_seq: &mut Self::TargetWidgetSeq) {
        if let Some(child) = self.as_ref() {
            if let Some(other_child) = other {
                child.reconcile(other_child, &mut widget_seq.as_mut().unwrap_or_log());
            } else {
                *widget_seq = Some(child.init_tree());
            }
        }
    }

    #[instrument(
        name = "Option",
        skip(self, component_state, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        component_state: &mut ComponentState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<ComponentEvent> {
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
        ComponentState,
        ComponentEvent,
        ChildLeft: ElementTree<ComponentState, ComponentEvent>,
        ChildRight: ElementTree<ComponentState, ComponentEvent>,
    > ElementTree<ComponentState, ComponentEvent> for Either<ChildLeft, ChildRight>
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
        ComponentState,
        ComponentEvent,
        ChildLeft: VirtualDom<ComponentState, ComponentEvent>,
        ChildRight: VirtualDom<ComponentState, ComponentEvent>,
    > VirtualDom<ComponentState, ComponentEvent> for Either<ChildLeft, ChildRight>
{
    type Event = NoEvent;
    type AggregateChildrenState =
        Option<Either<ChildLeft::AggregateChildrenState, ChildRight::AggregateChildrenState>>;
    type TargetWidgetSeq = Either<ChildLeft::TargetWidgetSeq, ChildRight::TargetWidgetSeq>;

    #[instrument(name = "Either", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Either", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        match self {
            Left(child) => Left(child.init_tree()),
            Right(child) => Right(child.init_tree()),
        }
    }

    #[instrument(name = "Either", skip(self, other, widget_seq))]
    fn reconcile(&self, other: &Self, widget_seq: &mut Self::TargetWidgetSeq) {
        match self {
            Left(child) => {
                if let Right(_) = &other {
                    *widget_seq = Left(child.init_tree());
                }
                child.reconcile(
                    other.as_ref().left().unwrap_or_log(),
                    &mut widget_seq.as_mut().left().unwrap_or_log(),
                );
            }
            Right(child) => {
                if let Left(_) = &other {
                    *widget_seq = Right(child.init_tree());
                }
                child.reconcile(
                    other.as_ref().right().unwrap_or_log(),
                    &mut widget_seq.as_mut().right().unwrap_or_log(),
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
        component_state: &mut ComponentState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<ComponentEvent> {
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
