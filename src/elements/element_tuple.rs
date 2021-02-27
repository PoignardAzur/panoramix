#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![rustfmt::skip]

use crate::element_tree::{Element, VirtualDom, NoEvent};
use crate::elements::EmptyElementData;
use crate::glue::GlobalEventCx;
use crate::widgets::WidgetTuple;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use tracing::instrument;

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound=""))]
pub struct ElementTupleData<
    C0: VirtualDom<CpState, CpEvent>,
    C1: VirtualDom<CpState, CpEvent>,
    C2: VirtualDom<CpState, CpEvent>,
    C3: VirtualDom<CpState, CpEvent>,
    C4: VirtualDom<CpState, CpEvent>,
    C5: VirtualDom<CpState, CpEvent>,
    C6: VirtualDom<CpState, CpEvent>,
    C7: VirtualDom<CpState, CpEvent>,
    C8: VirtualDom<CpState, CpEvent>,
    C9: VirtualDom<CpState, CpEvent>,
    C10: VirtualDom<CpState, CpEvent>,
    C11: VirtualDom<CpState, CpEvent>,
    CpState = (),
    CpEvent = NoEvent,
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
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

macro_rules! replace_ty {
    ($_t:tt >>> $sub:ty) => {$sub};
}

macro_rules! replace_expr {
    ($_t:tt >>> $sub:expr) => {$sub};
}

macro_rules! declare_stuff {
    ( $TupleName:ident ; $( $Type:ident ),* ; $( $Remainder:ident ),* ; $( $index:tt ),* ) => {

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound=""))]
pub struct $TupleName<
    $(
        $Type: Element<CpState, CpEvent>,
    )*
    CpState = (),
    CpEvent = NoEvent,
>(
    $(
        pub $Type,
    )*
    pub std::marker::PhantomData<CpState>,
    pub std::marker::PhantomData<CpEvent>,
);

impl<
        CpState,
        CpEvent,
        $(
            $Type: Element<CpState, CpEvent>,
        )*
    > Element<CpState, CpEvent> for $TupleName<$($Type,)* CpState, CpEvent>
{
    type Event = NoEvent;
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
            EmptyElementData<CpState, CpEvent>
        ),)*
        CpState,
        CpEvent,
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
            Default::default(),
            Default::default(),
        );

        (node, state)
    }
}

    };
}

declare_stuff!{
    ElementTuple_1;
    T0; __, __, __, __, __, __, __, __, __, __, __ ;
    0
}

declare_stuff!{
    ElementTuple_2;
    T0, T1; __, __, __, __, __, __, __, __, __, __ ;
    0, 1
}

declare_stuff!{
    ElementTuple_3;
    T0, T1, T2; __, __, __, __, __, __, __, __, __ ;
    0, 1, 2
}

declare_stuff!{
    ElementTuple_4;
    T0, T1, T2, T3; __, __, __, __, __, __, __, __ ;
    0, 1, 2, 3
}

declare_stuff!{
    ElementTuple_5;
    T0, T1, T2, T3, T4; __, __, __, __, __, __, __ ;
    0, 1, 2, 3, 4
}

declare_stuff!{
    ElementTuple_6;
    T0, T1, T2, T3, T4, T5; __, __, __, __, __, __ ;
    0, 1, 2, 3, 4, 5
}

declare_stuff!{
    ElementTuple_7;
    T0, T1, T2, T3, T4, T5, T6; __, __, __, __, __ ;
    0, 1, 2, 3, 4, 5, 6
}

declare_stuff!{
    ElementTuple_8;
    T0, T1, T2, T3, T4, T5, T6, T7; __, __, __, __ ;
    0, 1, 2, 3, 4, 5, 6, 7
}

declare_stuff!{
    ElementTuple_9;
    T0, T1, T2, T3, T4, T5, T6, T7, T8; __, __, __ ;
    0, 1, 2, 3, 4, 5, 6, 7, 8
}

declare_stuff!{
    ElementTuple_10;
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9; __, __ ;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9
}

declare_stuff!{
    ElementTuple_11;
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10; __ ;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
}

declare_stuff!{
    ElementTuple_12;
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11 ;;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
}

#[macro_export]
macro_rules! make_group {

    ( $(,)? ) => {
        $crate::elements::EmptyElement::new()
    };

    ( $e0:expr $(,)? ) => {
        $crate::elements::ElementTuple_1(
            $e0,
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr $(,)? ) => {
        $crate::elements::ElementTuple_2(
            $e0, $e1,
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr $(,)? ) => {
        $crate::elements::ElementTuple_3(
            $e0, $e1, $e2,
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr $(,)? ) => {
        $crate::elements::ElementTuple_4(
            $e0, $e1, $e2, $e3,
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr $(,)? ) => {
        $crate::elements::ElementTuple_5(
            $e0, $e1, $e2, $e3, $e4,
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr $(,)? ) => {
        $crate::elements::ElementTuple_6(
            $e0, $e1, $e2, $e3, $e4, $e5,
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr $(,)? ) => {
        $crate::elements::ElementTuple_7(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6,
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr $(,)? ) => {
        $crate::elements::ElementTuple_8(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7,
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr $(,)? ) => {
        $crate::elements::ElementTuple_9(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8,
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr, $e9:expr $(,)? ) => {
        $crate::elements::ElementTuple_10(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8, $e9,
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr, $e9:expr, $e10:expr $(,)? ) => {
        $crate::elements::ElementTuple_11(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8, $e9, $e10
            Default::default(), Default::default(),
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr, $e9:expr, $e10:expr, $e11:expr $(,)? ) => {
        $crate::elements::ElementTuple_12(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8, $e9, $e10, $e11
            Default::default(), Default::default(),
        )
    };

}

impl<
        CpState,
        CpEvent,
        C0: VirtualDom<CpState, CpEvent>,
        C1: VirtualDom<CpState, CpEvent>,
        C2: VirtualDom<CpState, CpEvent>,
        C3: VirtualDom<CpState, CpEvent>,
        C4: VirtualDom<CpState, CpEvent>,
        C5: VirtualDom<CpState, CpEvent>,
        C6: VirtualDom<CpState, CpEvent>,
        C7: VirtualDom<CpState, CpEvent>,
        C8: VirtualDom<CpState, CpEvent>,
        C9: VirtualDom<CpState, CpEvent>,
        C10: VirtualDom<CpState, CpEvent>,
        C11: VirtualDom<CpState, CpEvent>,
    > VirtualDom<CpState, CpEvent> for ElementTupleData<
        C0,
        C1,
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        C10,
        C11,
        CpState,
        CpEvent,
    >
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

    #[instrument(name = "Tuple", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

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

    #[instrument(name = "Tuple", skip(self, other, widget_seq, ctx))]
    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
            self.0.reconcile(&other.0, &mut widget_seq.0, ctx);
            self.1.reconcile(&other.1, &mut widget_seq.1, ctx);
            self.2.reconcile(&other.2, &mut widget_seq.2, ctx);
            self.3.reconcile(&other.3, &mut widget_seq.3, ctx);
            self.4.reconcile(&other.4, &mut widget_seq.4, ctx);
            self.5.reconcile(&other.5, &mut widget_seq.5, ctx);
            self.6.reconcile(&other.6, &mut widget_seq.6, ctx);
            self.7.reconcile(&other.7, &mut widget_seq.7, ctx);
            self.8.reconcile(&other.8, &mut widget_seq.8, ctx);
            self.9.reconcile(&other.9, &mut widget_seq.9, ctx);
            self.10.reconcile(&other.10, &mut widget_seq.10, ctx);
            self.11.reconcile(&other.11, &mut widget_seq.11, ctx);
    }

    #[instrument(name = "Tuple", skip(self, component_state, children_state, widget_seq, cx))]
    fn process_event(
        &self,
        component_state: &mut CpState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<CpEvent> {
        let event0 = self.0
            .process_event(component_state, &mut children_state.0, &mut widget_seq.0, cx);
        let event1 = self.1
            .process_event(component_state, &mut children_state.1, &mut widget_seq.1, cx);
        let event2 = self.2
            .process_event(component_state, &mut children_state.2, &mut widget_seq.2, cx);
        let event3 = self.3
            .process_event(component_state, &mut children_state.3, &mut widget_seq.3, cx);
        let event4 = self.4
            .process_event(component_state, &mut children_state.4, &mut widget_seq.4, cx);
        let event5 = self.5
            .process_event(component_state, &mut children_state.5, &mut widget_seq.5, cx);
        let event6 = self.6
            .process_event(component_state, &mut children_state.6, &mut widget_seq.6, cx);
        let event7 = self.7
            .process_event(component_state, &mut children_state.7, &mut widget_seq.7, cx);
        let event8 = self.8
            .process_event(component_state, &mut children_state.8, &mut widget_seq.8, cx);
        let event9 = self.9
            .process_event(component_state, &mut children_state.9, &mut widget_seq.9, cx);
        let event10 = self.10
            .process_event(component_state, &mut children_state.10, &mut widget_seq.10, cx);
        let event11 = self.11
            .process_event(component_state, &mut children_state.11, &mut widget_seq.11, cx);

        // FIXME - If several events happen simultaneously, this will swallow all but one
        // process_event() should return an iterator or an observable instead.
        None
            .or(event0)
            .or(event1)
            .or(event2)
            .or(event3)
            .or(event4)
            .or(event5)
            .or(event6)
            .or(event7)
            .or(event8)
            .or(event9)
            .or(event10)
            .or(event11)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::label::Label;
    use crate::element_tree::assign_empty_state_type;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn empty_tuple() {
        let tuple = make_group!();
        let tuple_data = tuple.clone().build(Default::default());

        assert_debug_snapshot!(tuple);
        assert_debug_snapshot!(tuple_data);

        assign_empty_state_type(&tuple);
    }

    #[test]
    fn new_tuple_single_item() {
        let tuple = make_group!(Label::new("Hello"));
        let tuple_data = tuple.clone().build(Default::default());

        assert_debug_snapshot!(tuple);
        assert_debug_snapshot!(tuple_data);

        assign_empty_state_type(&tuple);
    }

    #[test]
    fn new_tuple_multi_items() {
        let tuple = make_group!(
            Label::new("Hello"),
            Label::new("Hello2"),
            Label::new("Hello3")
        );
        let tuple_trailing_comma = make_group!(
            Label::new("Hello"),
            Label::new("Hello2"),
            Label::new("Hello3"),
        );
        let tuple_data = tuple.clone().build(Default::default());

        assert_debug_snapshot!(tuple);
        assert_debug_snapshot!(tuple_data);

        assert_eq!(tuple, tuple_trailing_comma);

        assign_empty_state_type(&tuple);
    }


    // TODO
    // - Add constructor
    // - Widget test
    // - Event test
}
