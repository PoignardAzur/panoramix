use crate::element_tree::{ElementTree, VirtualDom};
use crate::glue::GlobalEventCx;

use either::{Either, Left, Right};
use tracing::instrument;
use tracing_unwrap::OptionExt;

impl<ExplicitState, Child: ElementTree<ExplicitState>> ElementTree<ExplicitState>
    for Option<Child>
{
    type Event = Child::Event;
    type AggregateComponentState = Option<Child::AggregateComponentState>;
    type BuildOutput = Option<Child::BuildOutput>;

    #[instrument(name = "Option", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState) {
        if let Some(child) = self {
            let (output, state) = child.build(prev_state.unwrap_or_default());
            (Some(output), Some(state))
        } else {
            (None, None)
        }
    }
}

impl<Item: VirtualDom<ParentComponentState>, ParentComponentState> VirtualDom<ParentComponentState>
    for Option<Item>
{
    type Event = Item::Event;
    type AggregateComponentState = Option<Item::AggregateComponentState>;

    type DomState = Option<Item::DomState>;
    type TargetWidgetSeq = Option<Item::TargetWidgetSeq>;

    #[instrument(name = "Option", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Option", skip(self))]
    fn init_tree(&self) -> (Self::TargetWidgetSeq, Self::DomState) {
        if let Some(child) = self {
            let (widget_seq, state) = child.init_tree();
            (Some(widget_seq), Some(state))
        } else {
            (None, None)
        }
    }

    #[instrument(name = "Option", skip(self, other, prev_state, widget_seq))]
    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Self::DomState,
        widget_seq: &mut Self::TargetWidgetSeq,
    ) -> Self::DomState {
        let child = self.as_ref()?;

        if let Some(other_child) = other {
            Some(child.apply_diff(
                other_child,
                prev_state.unwrap_or_log(),
                &mut widget_seq.as_mut().unwrap_or_log(),
            ))
        } else {
            let (child_widget_seq, child_state) = child.init_tree();
            *widget_seq = Some(child_widget_seq);
            Some(child_state)
        }
    }

    #[instrument(
        name = "Option",
        skip(self, explicit_state, children_state, dom_state, cx)
    )]
    fn process_event(
        &self,
        explicit_state: &mut ParentComponentState,
        children_state: &mut Self::AggregateComponentState,
        dom_state: &mut Self::DomState,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        let child = self.as_ref()?;
        child.process_event(
            explicit_state,
            children_state.as_mut().unwrap_or_log(),
            dom_state.as_mut().unwrap_or_log(),
            cx,
        )
    }
}

// ----

impl<
        ExplicitState,
        ChildLeft: ElementTree<ExplicitState>,
        ChildRight: ElementTree<ExplicitState>,
    > ElementTree<ExplicitState> for Either<ChildLeft, ChildRight>
{
    type Event = Either<ChildLeft::Event, ChildRight::Event>;
    type AggregateComponentState =
        Option<Either<ChildLeft::AggregateComponentState, ChildRight::AggregateComponentState>>;
    type BuildOutput = Either<ChildLeft::BuildOutput, ChildRight::BuildOutput>;

    #[instrument(name = "Either", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState) {
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
        ItemLeft: VirtualDom<ParentComponentState>,
        ItemRight: VirtualDom<ParentComponentState>,
        ParentComponentState,
    > VirtualDom<ParentComponentState> for Either<ItemLeft, ItemRight>
{
    type Event = Either<ItemLeft::Event, ItemRight::Event>;
    type AggregateComponentState =
        Option<Either<ItemLeft::AggregateComponentState, ItemRight::AggregateComponentState>>;

    type DomState = Either<ItemLeft::DomState, ItemRight::DomState>;
    type TargetWidgetSeq = Either<ItemLeft::TargetWidgetSeq, ItemRight::TargetWidgetSeq>;

    #[instrument(name = "Either", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Either", skip(self))]
    fn init_tree(&self) -> (Self::TargetWidgetSeq, Self::DomState) {
        match self {
            Left(child) => {
                let (output, state) = child.init_tree();
                (Left(output), Left(state))
            }
            Right(child) => {
                let (output, state) = child.init_tree();
                (Right(output), Right(state))
            }
        }
    }

    #[instrument(name = "Either", skip(self, other, prev_state, widget_seq))]
    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Self::DomState,
        widget_seq: &mut Self::TargetWidgetSeq,
    ) -> Self::DomState {
        match self {
            Left(child) => {
                if let Right(_) = &other {
                    let (child_widget_seq, child_state) = child.init_tree();
                    *widget_seq = Left(child_widget_seq);
                    return Left(child_state);
                }
                Left(child.apply_diff(
                    other.as_ref().left().unwrap_or_log(),
                    prev_state.left().unwrap_or_log(),
                    &mut widget_seq.as_mut().left().unwrap_or_log(),
                ))
            }
            Right(child) => {
                if let Left(_) = &other {
                    let (child_widget_seq, child_state) = child.init_tree();
                    *widget_seq = Right(child_widget_seq);
                    return Right(child_state);
                }
                Right(child.apply_diff(
                    other.as_ref().right().unwrap_or_log(),
                    prev_state.right().unwrap_or_log(),
                    &mut widget_seq.as_mut().right().unwrap_or_log(),
                ))
            }
        }
    }

    #[instrument(
        name = "Either",
        skip(self, explicit_state, children_state, dom_state, cx)
    )]
    fn process_event(
        &self,
        explicit_state: &mut ParentComponentState,
        children_state: &mut Self::AggregateComponentState,
        dom_state: &mut Self::DomState,
        cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        match self {
            Left(child) => child
                .process_event(
                    explicit_state,
                    &mut children_state
                        .as_mut()
                        .unwrap_or_log()
                        .as_mut()
                        .left()
                        .unwrap_or_log(),
                    dom_state.as_mut().left().unwrap_or_log(),
                    cx,
                )
                .map(Left),
            Right(child) => child
                .process_event(
                    explicit_state,
                    &mut children_state
                        .as_mut()
                        .unwrap_or_log()
                        .as_mut()
                        .right()
                        .unwrap_or_log(),
                    dom_state.as_mut().right().unwrap_or_log(),
                    cx,
                )
                .map(Right),
        }
    }
}
