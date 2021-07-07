use crate::element_tree::ProcessEventCtx;
use crate::element_tree::ReconcileCtx;
use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::flex::{Axis, CrossAxisAlignment, FlexContainerParams, FlexParams, MainAxisAlignment};
use crate::glue::GlobalEventCx;
use crate::widgets::FlexWidget;
use crate::widgets::SingleWidget;

use derivative::Derivative;
use tracing::instrument;

#[derive(Derivative, PartialEq)]
#[derivative(Clone(bound = ""), Debug(bound = ""))]
pub struct Flex<Child: Element<CpEvent, CpState>, CpEvent = NoEvent, CpState = ()> {
    pub axis: Axis,
    pub child: Child,
    pub flex: FlexParams,
    pub flex_container: FlexContainerParams,
    pub _comp_state: std::marker::PhantomData<CpState>,
    pub _comp_event: std::marker::PhantomData<CpEvent>,
}

#[derive(Derivative, PartialEq)]
#[derivative(Clone(bound = "Child: Clone"), Debug(bound = ""))]
pub struct FlexData<Child: VirtualDom<CpEvent, CpState>, CpEvent = NoEvent, CpState = ()> {
    pub axis: Axis,
    pub child: Child,
    pub flex: FlexParams,
    pub flex_container: FlexContainerParams,
    pub _comp_state: std::marker::PhantomData<CpState>,
    pub _comp_event: std::marker::PhantomData<CpEvent>,
}

// ----

impl<CpEvent, CpState, Child: Element<CpEvent, CpState>> Flex<Child, CpEvent, CpState> {
    pub fn new(axis: Axis, child: Child) -> Self {
        Flex {
            axis,
            child,
            flex: FlexParams {
                flex: 1.0,
                alignment: None,
            },
            flex_container: FlexContainerParams {
                cross_alignment: CrossAxisAlignment::Center,
                main_alignment: MainAxisAlignment::Start,
                fill_major_axis: false,
            },
            _comp_state: Default::default(),
            _comp_event: Default::default(),
        }
    }

    pub fn with_flex_params(self, flex_params: FlexParams) -> Self {
        Flex {
            flex: flex_params,
            ..self
        }
    }

    pub fn with_flex_container_params(self, flex_container: FlexContainerParams) -> Self {
        Flex {
            flex_container,
            ..self
        }
    }
}

impl<CpEvent, CpState, Child: VirtualDom<CpEvent, CpState>> FlexData<Child, CpEvent, CpState> {
    pub fn new(
        axis: Axis,
        child: Child,
        flex: FlexParams,
        flex_container: FlexContainerParams,
    ) -> Self {
        FlexData {
            axis,
            child,
            flex,
            flex_container,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
        }
    }
}

// ----

impl<CpEvent, CpState, Child: Element<CpEvent, CpState>> Element<CpEvent, CpState>
    for Flex<Child, CpEvent, CpState>
{
    type Event = NoEvent;
    type ComponentState = crate::element_tree::NoState;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = FlexData<Child::BuildOutput, CpEvent, CpState>;

    #[instrument(name = "Flex", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (element, children_state) = self.child.build(prev_state);
        (
            FlexData::new(self.axis, element, self.flex, self.flex_container),
            children_state,
        )
    }
}

impl<CpEvent, CpState, Child: VirtualDom<CpEvent, CpState>> VirtualDom<CpEvent, CpState>
    for FlexData<Child, CpEvent, CpState>
{
    type Event = NoEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type TargetWidgetSeq = SingleWidget<FlexWidget<Child::TargetWidgetSeq>>;

    #[instrument(name = "Flex", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        let flex = FlexWidget {
            direction: self.axis,
            flex_params: self.flex_container,
            children_seq: self.child.init_tree(),
        };
        SingleWidget::new(flex, self.flex)
    }

    #[instrument(name = "Flex", skip(self, other, widget_seq, ctx))]
    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.child.reconcile(
            &other.child,
            &mut widget_seq.pod.widget_mut().children_seq,
            ctx,
        );
    }

    #[instrument(name = "Flex", skip(self, comp_ctx, children_state, widget_seq, cx))]
    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx<CpEvent, CpState>,
        children_state: &mut Child::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        self.child.process_event(
            comp_ctx,
            children_state,
            &mut widget_seq.pod.widget_mut().children_seq,
            cx,
        )
    }
}

// ----

// TODO - Add keyword params

/// Builds a row of up to 12 Elements.
///
/// Returns [`Flex`].
///
/// ## Events
///
/// Returned element doesn't emit events.
#[macro_export]
macro_rules! Row {
    ( $($arg:expr),* $(,)?) => {
        $crate::elements::Flex::new(
            $crate::flex::Axis::Horizontal,
            $crate::Tuple!($($arg,)*)
        )
    };
}

/// Builds a column of up to 12 Elements.
///
/// Returns [`Flex`].
///
/// ## Events
///
/// Returned element doesn't emit events.
#[macro_export]
macro_rules! Column {
    ( $($arg:expr),* $(,)?) => {
        $crate::elements::Flex::new(
            $crate::flex::Axis::Vertical,
            $crate::Tuple!($($arg,)*)
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element_tree::assign_empty_state_type;
    use crate::elements::Label;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn empty_rowcol() {
        let row = Row!();
        let column = Column!();
        let row_data = row.clone().build(Default::default());
        let column_data = column.clone().build(Default::default());

        assert_debug_snapshot!(row);
        assert_debug_snapshot!(column);
        assert_debug_snapshot!(row_data);
        assert_debug_snapshot!(column_data);

        assign_empty_state_type(&row);
        assign_empty_state_type(&column);
    }

    #[test]
    fn new_rowcol_single_item() {
        let row = Row!(Label::new("Hello"));
        let column = Column!(Label::new("Greetings"));
        let row_data = row.clone().build(Default::default());
        let column_data = column.clone().build(Default::default());

        assert_debug_snapshot!(row);
        assert_debug_snapshot!(column);
        assert_debug_snapshot!(row_data);
        assert_debug_snapshot!(column_data);

        assign_empty_state_type(&row);
        assign_empty_state_type(&column);
    }

    #[test]
    fn new_row_multi_items() {
        let row = Row!(
            Label::new("Hello"),
            Label::new("Hello2"),
            Label::new("Hello3"),
        );
        let row_data = row.clone().build(Default::default());

        assert_debug_snapshot!(row);
        assert_debug_snapshot!(row_data);

        assign_empty_state_type(&row);
    }

    #[test]
    fn rowcol_widget() {
        use crate::test_harness::Harness;

        let old_column = Column!(
            Row!(Label::new("Hello1"), Label::new("Hello2"),),
            Row!(Label::new("Hello3"), Label::new("Hello4"),),
            Label::new("Hello5"),
            Label::new("Hello6"),
        );
        let new_column = Column!(
            Row!(Label::new("World1"), Label::new("World2"),),
            Row!(Label::new("World3"), Label::new("World4"),),
            Label::new("World5"),
            Label::new("World6"),
        );

        Harness::run_test_window(old_column, |harness| {
            let column_state = harness.get_root_debug_state();
            assert_debug_snapshot!(column_state);

            harness.update_root_element(new_column.clone());

            let column_state_2 = harness.get_root_debug_state();
            assert_debug_snapshot!(column_state_2);
        });
    }

    // TODO
    // - Test that layout is calculated properly
}
