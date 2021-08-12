use crate::ctx::{ProcessEventCtx, ReconcileCtx};
use crate::element_tree::{Element, VirtualDom};
use crate::flex::{
    Axis, ContainerStyle, CrossAxisAlignment, FlexContainerParams, FlexParams, MainAxisAlignment,
};
use crate::glue::GlobalEventCx;
use crate::metadata::{NoEvent, NoState};
use crate::widgets::{Container, FlexWidget, SingleWidget};

use druid::KeyOrValue;

use tracing::instrument;

/// A flex container, either horizontal or vertical.
#[derive(Clone, Debug, PartialEq)]
pub struct Flex<Child: Element> {
    pub axis: Axis,
    pub child: Child,
    pub flex: FlexParams,
    pub flex_container: FlexContainerParams,
    pub container_style: ContainerStyle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlexData<Child: VirtualDom> {
    pub axis: Axis,
    pub child: Child,
    pub flex: FlexParams,
    pub flex_container: FlexContainerParams,
    pub container_style: ContainerStyle,
}

// ----

impl<Child: Element> Flex<Child> {
    pub fn new(axis: Axis, child: Child) -> Self {
        Flex {
            axis,
            child,
            flex: FlexParams {
                flex: None,
                alignment: None,
            },
            flex_container: FlexContainerParams {
                cross_alignment: CrossAxisAlignment::Center,
                main_alignment: MainAxisAlignment::Start,
                fill_major_axis: false,
            },
            container_style: ContainerStyle {
                background: None,
                border: None,
                corner_radius: KeyOrValue::Concrete(0.0),
            },
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

    pub fn with_container_style(self, container_style: ContainerStyle) -> Self {
        Flex {
            container_style,
            ..self
        }
    }
}

impl<Child: VirtualDom> FlexData<Child> {
    pub fn new(
        axis: Axis,
        child: Child,
        flex: FlexParams,
        flex_container: FlexContainerParams,
        container_style: ContainerStyle,
    ) -> Self {
        FlexData {
            axis,
            child,
            flex,
            flex_container,
            container_style,
        }
    }
}

// ----

impl<Child: Element> Element for Flex<Child> {
    type Event = NoEvent;
    type ComponentState = NoState;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type BuildOutput = FlexData<Child::BuildOutput>;

    #[instrument(name = "Flex", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let (element, children_state) = self.child.build(prev_state);
        (
            FlexData::new(
                self.axis,
                element,
                self.flex,
                self.flex_container,
                self.container_style,
            ),
            children_state,
        )
    }
}

impl<Child: VirtualDom> VirtualDom for FlexData<Child> {
    type Event = NoEvent;
    type AggregateChildrenState = Child::AggregateChildrenState;
    type TargetWidgetSeq =
        SingleWidget<Container<crate::glue::DruidAppData, FlexWidget<Child::TargetWidgetSeq>>>;

    #[instrument(name = "Flex", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        let flex = FlexWidget {
            direction: self.axis,
            flex_params: self.flex_container,
            children_seq: self.child.init_tree(),
        };
        let mut container = Container::new(flex);
        if let Some(KeyOrValue::Key(background)) = &self.container_style.background {
            container.set_background(background.clone());
        }
        if let Some(KeyOrValue::Concrete(background)) = &self.container_style.background {
            container.set_background(background.clone());
        }
        if let Some(border) = self.container_style.border.clone() {
            container.set_border(border.color, border.width);
        }
        container.set_rounded(self.container_style.corner_radius.clone());
        SingleWidget::new(container, self.flex)
    }

    #[instrument(name = "Flex", skip(self, prev_value, widget_seq, ctx))]
    fn reconcile(
        &self,
        prev_value: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        // TODO - Reconcile style params
        self.child.reconcile(
            &prev_value.child,
            &mut widget_seq.pod.widget_mut().child_mut().children_seq,
            ctx,
        );
    }

    #[instrument(name = "Flex", skip(self, comp_ctx, children_state, widget_seq, cx))]
    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
        children_state: &mut Child::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        self.child.process_event(
            comp_ctx,
            children_state,
            &mut widget_seq.pod.widget_mut().child_mut().children_seq,
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
///
/// ## Example
///
/// ```rust
/// # use panoramix::{Row};
/// # use panoramix::elements::{Label, Button};
/// let row = Row!(
///     Label::new("Hello"),
///     Label::new("World"),
///     Button::new("Click me!")
/// );
/// ```
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
///
/// ## Example
///
/// ```rust
/// # use panoramix::{Column};
/// # use panoramix::elements::{Label, Button};
/// let column = Column!(
///     Label::new("Hello"),
///     Label::new("World"),
///     Button::new("Click me!")
/// );
/// ```
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
