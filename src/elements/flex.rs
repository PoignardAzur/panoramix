use crate::glue::GlobalEventCx;

use crate::element_tree::{ElementTree, VirtualDom};
use crate::widgets::SingleWidget;

use crate::widgets::flex::Axis;
use crate::widgets::flex::CrossAxisAlignment;
use crate::widgets::flex::Flex;
use crate::widgets::flex::MainAxisAlignment;
use derivative::Derivative;

// TODO - merge row and column

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""))]
pub struct Row<Child: ElementTree<ExplicitState>, ExplicitState = ()> {
    pub child: Child,
    pub _expl_state: std::marker::PhantomData<ExplicitState>,
}

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""))]
pub struct RowData<Item: VirtualDom<ParentComponentState>, ParentComponentState> {
    pub child: Item,
    pub _expl_state: std::marker::PhantomData<ParentComponentState>,
}

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""))]
pub struct Column<Child: ElementTree<ExplicitState>, ExplicitState = ()> {
    pub child: Child,
    pub _expl_state: std::marker::PhantomData<ExplicitState>,
}

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""))]
pub struct ColumnData<Item: VirtualDom<ParentComponentState>, ParentComponentState> {
    pub child: Item,
    pub _expl_state: std::marker::PhantomData<ParentComponentState>,
}

// ----

impl<ExplicitState, Child: ElementTree<ExplicitState>> Row<Child, ExplicitState> {
    pub fn new(child: Child) -> Self {
        Row {
            child,
            _expl_state: Default::default(),
        }
    }
}

impl<Item: VirtualDom<ParentComponentState>, ParentComponentState>
    RowData<Item, ParentComponentState>
{
    pub fn new(child: Item) -> Self {
        RowData {
            child,
            _expl_state: Default::default(),
        }
    }
}

impl<ExplicitState, Child: ElementTree<ExplicitState>> Column<Child, ExplicitState> {
    pub fn new(child: Child) -> Self {
        Column {
            child,
            _expl_state: Default::default(),
        }
    }
}

impl<Item: VirtualDom<ParentComponentState>, ParentComponentState>
    ColumnData<Item, ParentComponentState>
{
    pub fn new(child: Item) -> Self {
        ColumnData {
            child,
            _expl_state: Default::default(),
        }
    }
}

impl<ExplicitState, Child: ElementTree<ExplicitState>> ElementTree<ExplicitState>
    for Row<Child, ExplicitState>
{
    type Event = Child::Event;
    type AggregateComponentState = Child::AggregateComponentState;
    type BuildOutput = RowData<Child::BuildOutput, ExplicitState>;

    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState) {
        let (element, component_state) = self.child.build(prev_state);
        (RowData::new(element), component_state)
    }
}

impl<Item: VirtualDom<ParentComponentState>, ParentComponentState> VirtualDom<ParentComponentState>
    for RowData<Item, ParentComponentState>
{
    type Event = Item::Event;
    type DomState = Item::DomState;
    type AggregateComponentState = Item::AggregateComponentState;

    type TargetWidgetSeq = SingleWidget<Flex<Item::TargetWidgetSeq>>;

    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    fn init_tree(&self) -> (Self::TargetWidgetSeq, Item::DomState) {
        let (widget_seq, dom_state) = self.child.init_tree();

        // FIXME - Pull params from constructor
        let flex = Flex {
            direction: Axis::Horizontal,
            cross_alignment: CrossAxisAlignment::Center,
            main_alignment: MainAxisAlignment::Start,
            fill_major_axis: false,
            children_seq: widget_seq,
        };
        (SingleWidget::new(flex), dom_state)
    }

    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Item::DomState,
        widget: &mut Self::TargetWidgetSeq,
    ) -> Item::DomState {
        self.child.apply_diff(
            &other.child,
            prev_state,
            &mut widget.0.widget_mut().children_seq,
        )
    }

    fn process_event(
        &self,
        explicit_state: &mut ParentComponentState,
        children_state: &mut Item::AggregateComponentState,
        dom_state: &mut Item::DomState,
        cx: &mut GlobalEventCx,
    ) -> Option<Item::Event> {
        self.child
            .process_event(explicit_state, children_state, dom_state, cx)
    }
}

// ----

impl<ExplicitState, Child: ElementTree<ExplicitState>> ElementTree<ExplicitState>
    for Column<Child, ExplicitState>
{
    type Event = Child::Event;
    type AggregateComponentState = Child::AggregateComponentState;
    type BuildOutput = ColumnData<Child::BuildOutput, ExplicitState>;

    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState) {
        let (element, component_state) = self.child.build(prev_state);
        (ColumnData::new(element), component_state)
    }
}

impl<Item: VirtualDom<ParentComponentState>, ParentComponentState> VirtualDom<ParentComponentState>
    for ColumnData<Item, ParentComponentState>
{
    type Event = Item::Event;
    type DomState = Item::DomState;
    type AggregateComponentState = Item::AggregateComponentState;

    type TargetWidgetSeq = SingleWidget<Flex<Item::TargetWidgetSeq>>;

    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    fn init_tree(&self) -> (Self::TargetWidgetSeq, Item::DomState) {
        let (widget_seq, dom_state) = self.child.init_tree();

        // FIXME - Pull params from constructor
        let flex = Flex {
            direction: Axis::Vertical,
            cross_alignment: CrossAxisAlignment::Center,
            main_alignment: MainAxisAlignment::Start,
            fill_major_axis: false,
            children_seq: widget_seq,
        };
        (SingleWidget::new(flex), dom_state)
    }

    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Item::DomState,
        widget: &mut Self::TargetWidgetSeq,
    ) -> Item::DomState {
        self.child.apply_diff(
            &other.child,
            prev_state,
            &mut widget.0.widget_mut().children_seq,
        )
    }

    fn process_event(
        &self,
        explicit_state: &mut ParentComponentState,
        children_state: &mut Item::AggregateComponentState,
        dom_state: &mut Item::DomState,
        cx: &mut GlobalEventCx,
    ) -> Option<Item::Event> {
        self.child
            .process_event(explicit_state, children_state, dom_state, cx)
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
