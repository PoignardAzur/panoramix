use crate::element_tree::ReconcileCtx;
use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;
use crate::widgets::flex::{
    Axis, CrossAxisAlignment, FlexContainerParams, FlexParams, FlexWidget, MainAxisAlignment,
};
use crate::widgets::SingleWidget;

use derivative::Derivative;
use tracing::instrument;

#[derive(Derivative, Clone, PartialEq)]
#[derivative(Debug(bound = ""))]
pub struct Flex<Child: Element<CpState, CpEvent>, CpState = (), CpEvent = NoEvent> {
    pub axis: Axis,
    pub child: Child,
    pub flex: FlexParams,
    pub flex_container: FlexContainerParams,
    pub _comp_state: std::marker::PhantomData<CpState>,
    pub _comp_event: std::marker::PhantomData<CpEvent>,
}

#[derive(Derivative, Clone, PartialEq)]
#[derivative(Debug(bound = ""))]
pub struct FlexData<Child: VirtualDom<CpState, CpEvent>, CpState = (), CpEvent = NoEvent> {
    pub axis: Axis,
    pub child: Child,
    pub flex: FlexParams,
    pub flex_container: FlexContainerParams,
    pub _comp_state: std::marker::PhantomData<CpState>,
    pub _comp_event: std::marker::PhantomData<CpEvent>,
}

// ----

impl<CpState, CpEvent, Child: Element<CpState, CpEvent>> Flex<Child, CpState, CpEvent> {
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

impl<CpState, CpEvent, Child: VirtualDom<CpState, CpEvent>> FlexData<Child, CpState, CpEvent> {
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

impl<CpState, CpEvent, Child: Element<CpState, CpEvent>> Element<CpState, CpEvent>
    for Flex<Child, CpState, CpEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = FlexData<Child::BuildOutput, CpState, CpEvent>;

    #[instrument(name = "Flex", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (element, component_state) = self.child.build(prev_state);
        (
            FlexData::new(self.axis, element, self.flex, self.flex_container),
            component_state,
        )
    }
}

impl<CpState, CpEvent, Child: VirtualDom<CpState, CpEvent>> VirtualDom<CpState, CpEvent>
    for FlexData<Child, CpState, CpEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;

    type TargetWidgetSeq = SingleWidget<FlexWidget<Child::TargetWidgetSeq>>;

    #[instrument(name = "Flex", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

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

    #[instrument(
        name = "Flex",
        skip(self, component_state, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        component_state: &mut CpState,
        children_state: &mut Child::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<CpEvent> {
        self.child.process_event(
            component_state,
            children_state,
            &mut widget_seq.pod.widget_mut().children_seq,
            cx,
        )
    }
}

// ----

// TODO - Add keyword params

#[macro_export]
macro_rules! Row {
    ( $($arg:expr),* $(,)?) => {
        $crate::elements::Flex::new(
            $crate::widgets::flex::Axis::Horizontal,
            $crate::Tuple!($($arg,)*)
        )
    };
}

#[macro_export]
macro_rules! Column {
    ( $($arg:expr),* $(,)?) => {
        $crate::elements::Flex::new(
            $crate::widgets::flex::Axis::Vertical,
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

    // TODO
    // - Id test (??)
    // - Event test
    // - Widget test
}
