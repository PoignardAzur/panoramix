use crate::element_tree::{ElementTree, NoEvent, VirtualDom};
use crate::glue::GlobalEventCx;
use crate::widgets::flex::{
    Axis, CrossAxisAlignment, Flex, FlexContainerParams, FlexParams, MainAxisAlignment,
};
use crate::widgets::SingleWidget;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use tracing::instrument;

// TODO - merge row and column

#[derive(Derivative, Clone, PartialEq)]
#[derivative(Debug(bound = ""))]
pub struct Row<
    Child: ElementTree<ComponentState, ComponentEvent>,
    ComponentState = (),
    ComponentEvent = NoEvent,
> {
    pub child: Child,
    pub flex: FlexParams,
    pub flex_container: FlexContainerParams,
    pub _comp_state: std::marker::PhantomData<ComponentState>,
    pub _comp_event: std::marker::PhantomData<ComponentEvent>,
}

#[derive(Derivative, Clone, PartialEq)]
#[derivative(Debug(bound = ""))]
pub struct RowData<
    Child: VirtualDom<ComponentState, ComponentEvent>,
    ComponentState = (),
    ComponentEvent = NoEvent,
> {
    pub child: Child,
    pub flex: FlexParams,
    pub flex_container: FlexContainerParams,
    pub _comp_state: std::marker::PhantomData<ComponentState>,
    pub _comp_event: std::marker::PhantomData<ComponentEvent>,
}

#[derive(Derivative, Clone, PartialEq)]
#[derivative(Debug(bound = ""))]
pub struct Column<
    Child: ElementTree<ComponentState, ComponentEvent>,
    ComponentState = (),
    ComponentEvent = NoEvent,
> {
    pub child: Child,
    pub flex: FlexParams,
    pub flex_container: FlexContainerParams,
    pub _comp_state: std::marker::PhantomData<ComponentState>,
    pub _comp_event: std::marker::PhantomData<ComponentEvent>,
}

#[derive(Derivative, Clone, PartialEq)]
#[derivative(Debug(bound = ""))]
pub struct ColumnData<
    Child: VirtualDom<ComponentState, ComponentEvent>,
    ComponentState = (),
    ComponentEvent = NoEvent,
> {
    pub child: Child,
    pub flex: FlexParams,
    pub flex_container: FlexContainerParams,
    pub _comp_state: std::marker::PhantomData<ComponentState>,
    pub _comp_event: std::marker::PhantomData<ComponentEvent>,
}

// ----

impl<ComponentState, ComponentEvent, Child: ElementTree<ComponentState, ComponentEvent>>
    Row<Child, ComponentState, ComponentEvent>
{
    pub fn new(child: Child) -> Self {
        Row {
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
        Row {
            flex: flex_params,
            ..self
        }
    }

    pub fn with_flex_container_params(self, flex_container: FlexContainerParams) -> Self {
        Row {
            flex_container,
            ..self
        }
    }
}

impl<ComponentState, ComponentEvent, Child: VirtualDom<ComponentState, ComponentEvent>>
    RowData<Child, ComponentState, ComponentEvent>
{
    pub fn new(child: Child, flex: FlexParams, flex_container: FlexContainerParams) -> Self {
        RowData {
            child,
            flex,
            flex_container,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
        }
    }
}

impl<ComponentState, ComponentEvent, Child: ElementTree<ComponentState, ComponentEvent>>
    Column<Child, ComponentState, ComponentEvent>
{
    pub fn new(child: Child) -> Self {
        Column {
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
        Column {
            flex: flex_params,
            ..self
        }
    }

    pub fn with_flex_container_params(self, flex_container: FlexContainerParams) -> Self {
        Column {
            flex_container,
            ..self
        }
    }
}

impl<ComponentState, ComponentEvent, Child: VirtualDom<ComponentState, ComponentEvent>>
    ColumnData<Child, ComponentState, ComponentEvent>
{
    pub fn new(child: Child, flex: FlexParams, flex_container: FlexContainerParams) -> Self {
        ColumnData {
            child,
            flex,
            flex_container,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
        }
    }
}

impl<ComponentState, ComponentEvent, Child: ElementTree<ComponentState, ComponentEvent>>
    ElementTree<ComponentState, ComponentEvent> for Row<Child, ComponentState, ComponentEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = RowData<Child::BuildOutput, ComponentState, ComponentEvent>;

    #[instrument(name = "Flex", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (element, component_state) = self.child.build(prev_state);
        (
            RowData::new(element, self.flex, self.flex_container),
            component_state,
        )
    }
}

impl<ComponentState, ComponentEvent, Child: VirtualDom<ComponentState, ComponentEvent>>
    VirtualDom<ComponentState, ComponentEvent> for RowData<Child, ComponentState, ComponentEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;

    type TargetWidgetSeq = SingleWidget<Flex<Child::TargetWidgetSeq>>;

    #[instrument(name = "Flex", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Flex", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        // FIXME - Pull params from constructor
        let flex = Flex {
            direction: Axis::Horizontal,
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
        component_state: &mut ComponentState,
        children_state: &mut Child::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<ComponentEvent> {
        self.child.process_event(
            component_state,
            children_state,
            &mut widget_seq.pod.widget_mut().children_seq,
            cx,
        )
    }
}

// ----

impl<ComponentState, ComponentEvent, Child: ElementTree<ComponentState, ComponentEvent>>
    ElementTree<ComponentState, ComponentEvent> for Column<Child, ComponentState, ComponentEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = ColumnData<Child::BuildOutput, ComponentState, ComponentEvent>;

    #[instrument(name = "Flex", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (element, component_state) = self.child.build(prev_state);
        (
            ColumnData::new(element, self.flex, self.flex_container),
            component_state,
        )
    }
}

impl<Child: VirtualDom<ComponentState, ComponentEvent>, ComponentState, ComponentEvent>
    VirtualDom<ComponentState, ComponentEvent>
    for ColumnData<Child, ComponentState, ComponentEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type TargetWidgetSeq = SingleWidget<Flex<Child::TargetWidgetSeq>>;

    #[instrument(name = "Flex", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Flex", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        // FIXME - Pull params from constructor
        let flex = Flex {
            direction: Axis::Vertical,
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
        component_state: &mut ComponentState,
        children_state: &mut Child::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<ComponentEvent> {
        self.child.process_event(
            component_state,
            children_state,
            &mut widget_seq.pod.widget_mut().children_seq,
            cx,
        )
    }
}

#[macro_export]
macro_rules! make_row {
    ( $($arg:expr),* $(,)?) => {
        $crate::elements::Row::new(
            $crate::make_group!($($arg,)*)
        )
    };
}

#[macro_export]
macro_rules! make_column {
    ( $($arg:expr),* $(,)?) => {
        $crate::elements::Column::new(
            $crate::make_group!($($arg,)*)
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
        let row = make_row!();
        let column = make_column!();
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
        let row = make_row!(Label::new("Hello"));
        let column = make_column!(Label::new("Greetings"));
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
        let row = make_row!(
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
