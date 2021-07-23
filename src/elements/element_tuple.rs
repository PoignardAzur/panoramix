#![allow(unused_attributes)]
#![allow(non_camel_case_types)]

use crate::ctx::{ProcessEventCtx, ReconcileCtx};
use crate::element_tree::{Element, VirtualDom};
use crate::elements::internals::EmptyElementData;
use crate::glue::GlobalEventCx;
use crate::metadata::{NoEvent, NoState};
use crate::widgets::WidgetTuple;

use tracing::instrument;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ElementTupleData<
    C0: VirtualDom,
    C1: VirtualDom,
    C2: VirtualDom,
    C3: VirtualDom,
    C4: VirtualDom,
    C5: VirtualDom,
    C6: VirtualDom,
    C7: VirtualDom,
    C8: VirtualDom,
    C9: VirtualDom,
    C10: VirtualDom,
    C11: VirtualDom,
>(
    pub C0,
    pub C1,
    pub C2,
    pub C3,
    pub C4,
    pub C5,
    pub C6,
    pub C7,
    pub C8,
    pub C9,
    pub C10,
    pub C11,
);

macro_rules! replace_ty {
    ($_t:tt >>> $sub:ty) => {
        $sub
    };
}

macro_rules! replace_expr {
    ($_t:tt >>> $sub:expr) => {
        $sub
    };
}

macro_rules! declare_stuff {
    ( $TupleName:ident ; $( $Type:ident ),* ; $( $Remainder:ident ),* ; $( $index:tt ),* ) => {

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct $TupleName<
    $(
        $Type: Element,
    )*
>(
    $(
        pub $Type,
    )*
);

impl<
        $(
            $Type: Element,
        )*
    > Element for $TupleName<$($Type,)*>
{
    type Event = NoEvent;
    type ComponentState = NoState;
    type AggregateChildrenState = (
        $(
            $Type::AggregateChildrenState,
        )*
        $(replace_ty!(($Remainder) >>>
            ()
        ),)*
    );
    type BuildOutput = ElementTupleData<
        $(
            $Type::BuildOutput,
        )*
        $(replace_ty!(($Remainder) >>>
            EmptyElementData
        ),)*
    >;

    #[instrument(name = "Tuple", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        let mut state : Self::AggregateChildrenState = Default::default();

        let node = ElementTupleData(
            $(
                {
                    let (subnode, substate) = self.$index.build(prev_state.$index);
                    state.$index = substate;
                    subnode
                },
            )*
            $(replace_expr!(($Remainder) >>>
                Default::default()
            ),)*
        );

        (node, state)
    }
}

    };
}

declare_stuff! {
    ElementTuple_1;
    T0; __, __, __, __, __, __, __, __, __, __, __ ;
    0
}

declare_stuff! {
    ElementTuple_2;
    T0, T1; __, __, __, __, __, __, __, __, __, __ ;
    0, 1
}

declare_stuff! {
    ElementTuple_3;
    T0, T1, T2; __, __, __, __, __, __, __, __, __ ;
    0, 1, 2
}

declare_stuff! {
    ElementTuple_4;
    T0, T1, T2, T3; __, __, __, __, __, __, __, __ ;
    0, 1, 2, 3
}

declare_stuff! {
    ElementTuple_5;
    T0, T1, T2, T3, T4; __, __, __, __, __, __, __ ;
    0, 1, 2, 3, 4
}

declare_stuff! {
    ElementTuple_6;
    T0, T1, T2, T3, T4, T5; __, __, __, __, __, __ ;
    0, 1, 2, 3, 4, 5
}

declare_stuff! {
    ElementTuple_7;
    T0, T1, T2, T3, T4, T5, T6; __, __, __, __, __ ;
    0, 1, 2, 3, 4, 5, 6
}

declare_stuff! {
    ElementTuple_8;
    T0, T1, T2, T3, T4, T5, T6, T7; __, __, __, __ ;
    0, 1, 2, 3, 4, 5, 6, 7
}

declare_stuff! {
    ElementTuple_9;
    T0, T1, T2, T3, T4, T5, T6, T7, T8; __, __, __ ;
    0, 1, 2, 3, 4, 5, 6, 7, 8
}

declare_stuff! {
    ElementTuple_10;
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9; __, __ ;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9
}

declare_stuff! {
    ElementTuple_11;
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10; __ ;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
}

declare_stuff! {
    ElementTuple_12;
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11 ;;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
}

/// Builds a group of up to 12 Elements, without a specified layout.
///
/// Return value implements [`Element`](crate::Element) as well.
///
/// ## Events
///
/// Returned element doesn't emit events.
#[macro_export]
macro_rules! Tuple {
    ( $(,)? ) => {
        $crate::elements::EmptyElement::new()
    };

    ( $e0:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_1($e0)
    };
    ( $e0:expr, $e1:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_2($e0, $e1)
    };
    ( $e0:expr, $e1:expr, $e2:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_3($e0, $e1, $e2)
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_4($e0, $e1, $e2, $e3)
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_5($e0, $e1, $e2, $e3, $e4)
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_6($e0, $e1, $e2, $e3, $e4, $e5)
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_7($e0, $e1, $e2, $e3, $e4, $e5, $e6)
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_8($e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7)
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_9($e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8)
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr, $e9:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_10(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8, $e9,
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr, $e9:expr, $e10:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_11(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8, $e9, $e10,
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr, $e9:expr, $e10:expr, $e11:expr $(,)? ) => {
        $crate::elements::element_tuple::ElementTuple_12(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8, $e9, $e10, $e11,
        )
    };
}

impl<
        C0: VirtualDom,
        C1: VirtualDom,
        C2: VirtualDom,
        C3: VirtualDom,
        C4: VirtualDom,
        C5: VirtualDom,
        C6: VirtualDom,
        C7: VirtualDom,
        C8: VirtualDom,
        C9: VirtualDom,
        C10: VirtualDom,
        C11: VirtualDom,
    > VirtualDom for ElementTupleData<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11>
{
    type Event = NoEvent;
    type AggregateChildrenState = (
        C0::AggregateChildrenState,
        C1::AggregateChildrenState,
        C2::AggregateChildrenState,
        C3::AggregateChildrenState,
        C4::AggregateChildrenState,
        C5::AggregateChildrenState,
        C6::AggregateChildrenState,
        C7::AggregateChildrenState,
        C8::AggregateChildrenState,
        C9::AggregateChildrenState,
        C10::AggregateChildrenState,
        C11::AggregateChildrenState,
    );
    type TargetWidgetSeq = WidgetTuple<
        C0::TargetWidgetSeq,
        C1::TargetWidgetSeq,
        C2::TargetWidgetSeq,
        C3::TargetWidgetSeq,
        C4::TargetWidgetSeq,
        C5::TargetWidgetSeq,
        C6::TargetWidgetSeq,
        C7::TargetWidgetSeq,
        C8::TargetWidgetSeq,
        C9::TargetWidgetSeq,
        C10::TargetWidgetSeq,
        C11::TargetWidgetSeq,
    >;

    #[instrument(name = "Tuple", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        WidgetTuple(
            self.0.init_tree(),
            self.1.init_tree(),
            self.2.init_tree(),
            self.3.init_tree(),
            self.4.init_tree(),
            self.5.init_tree(),
            self.6.init_tree(),
            self.7.init_tree(),
            self.8.init_tree(),
            self.9.init_tree(),
            self.10.init_tree(),
            self.11.init_tree(),
        )
    }

    #[instrument(name = "Tuple", skip(self, prev_value, widget_seq, ctx))]
    fn reconcile(
        &self,
        prev_value: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.0.reconcile(&prev_value.0, &mut widget_seq.0, ctx);
        self.1.reconcile(&prev_value.1, &mut widget_seq.1, ctx);
        self.2.reconcile(&prev_value.2, &mut widget_seq.2, ctx);
        self.3.reconcile(&prev_value.3, &mut widget_seq.3, ctx);
        self.4.reconcile(&prev_value.4, &mut widget_seq.4, ctx);
        self.5.reconcile(&prev_value.5, &mut widget_seq.5, ctx);
        self.6.reconcile(&prev_value.6, &mut widget_seq.6, ctx);
        self.7.reconcile(&prev_value.7, &mut widget_seq.7, ctx);
        self.8.reconcile(&prev_value.8, &mut widget_seq.8, ctx);
        self.9.reconcile(&prev_value.9, &mut widget_seq.9, ctx);
        self.10.reconcile(&prev_value.10, &mut widget_seq.10, ctx);
        self.11.reconcile(&prev_value.11, &mut widget_seq.11, ctx);
    }

    #[rustfmt::skip]
    #[instrument(name = "Tuple", skip(self, comp_ctx, children_state, widget_seq, cx))]
    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        self.0.process_event(comp_ctx, &mut children_state.0, &mut widget_seq.0, cx);
        self.1.process_event(comp_ctx, &mut children_state.1, &mut widget_seq.1, cx);
        self.2.process_event(comp_ctx, &mut children_state.2, &mut widget_seq.2, cx);
        self.3.process_event(comp_ctx, &mut children_state.3, &mut widget_seq.3, cx);
        self.4.process_event(comp_ctx, &mut children_state.4, &mut widget_seq.4, cx);
        self.5.process_event(comp_ctx, &mut children_state.5, &mut widget_seq.5, cx);
        self.6.process_event(comp_ctx, &mut children_state.6, &mut widget_seq.6, cx);
        self.7.process_event(comp_ctx, &mut children_state.7, &mut widget_seq.7, cx);
        self.8.process_event(comp_ctx, &mut children_state.8, &mut widget_seq.8, cx);
        self.9.process_event(comp_ctx, &mut children_state.9, &mut widget_seq.9, cx);
        self.10.process_event(comp_ctx, &mut children_state.10, &mut widget_seq.10, cx);
        self.11.process_event(comp_ctx, &mut children_state.11, &mut widget_seq.11, cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::label::Label;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn empty_tuple() {
        let tuple = Tuple!();
        let tuple_data = tuple.clone().build(Default::default());

        assert_debug_snapshot!(tuple);
        assert_debug_snapshot!(tuple_data);
    }

    #[test]
    fn new_tuple_single_item() {
        let tuple = Tuple!(Label::new("Hello"));
        let tuple_data = tuple.clone().build(Default::default());

        assert_debug_snapshot!(tuple);
        assert_debug_snapshot!(tuple_data);
    }

    #[test]
    fn new_tuple_multi_items() {
        let tuple = Tuple!(
            Label::new("Hello"),
            Label::new("Hello2"),
            Label::new("Hello3")
        );
        let tuple_trailing_comma = Tuple!(
            Label::new("Hello"),
            Label::new("Hello2"),
            Label::new("Hello3"),
        );
        let tuple_data = tuple.clone().build(Default::default());

        assert_debug_snapshot!(tuple);
        assert_debug_snapshot!(tuple_data);

        assert_eq!(tuple, tuple_trailing_comma);
    }

    #[test]
    fn label_tuple_widget() {
        use crate::test_harness::Harness;

        let tuple = Tuple!(
            Label::new("Hello"),
            Label::new("Hello2"),
            Label::new("Hello3"),
        );

        Harness::run_test_window(tuple, |harness| {
            let tuple_state = harness.get_root_debug_state();
            assert_debug_snapshot!(tuple_state);

            let new_tuple = Tuple!(
                Label::new("World"),
                Label::new("World2"),
                Label::new("World3"),
            );
            harness.update_root_element(new_tuple);

            let tuple_state_2 = harness.get_root_debug_state();
            assert_debug_snapshot!(tuple_state_2);
        });
    }
}
