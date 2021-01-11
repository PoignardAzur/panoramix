#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![rustfmt::skip]

use derivative::Derivative;

use crate::glue::GlobalEventCx;
use crate::widgets::WidgetTuple;

use crate::element_tree::{ElementTree, VirtualDom};

use crate::elements::EmptyElementData;

#[derive(Derivative, Clone, Default, PartialEq, Eq, Hash)]
#[derivative(Debug(bound=""))]
pub struct ElementTupleData<
    C0: VirtualDom<ParentComponentState>,
    C1: VirtualDom<ParentComponentState>,
    C2: VirtualDom<ParentComponentState>,
    C3: VirtualDom<ParentComponentState>,
    C4: VirtualDom<ParentComponentState>,
    C5: VirtualDom<ParentComponentState>,
    C6: VirtualDom<ParentComponentState>,
    C7: VirtualDom<ParentComponentState>,
    C8: VirtualDom<ParentComponentState>,
    C9: VirtualDom<ParentComponentState>,
    C10: VirtualDom<ParentComponentState>,
    C11: VirtualDom<ParentComponentState>,
    ParentComponentState,
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
    pub std::marker::PhantomData<ParentComponentState>,
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
        $Type: ElementTree<ExplicitState>,
    )*
    ExplicitState = (),
>(
    $(
        pub $Type,
    )*
    pub std::marker::PhantomData<ExplicitState>,
);

impl<
        ExplicitState,
        $(
            $Type: ElementTree<ExplicitState>,
        )*
    > ElementTree<ExplicitState> for $TupleName<$($Type,)* ExplicitState>
{
    type Event = EventEnum<
        $(
            $Type::Event,
        )*
        $(replace_ty!(($Remainder) >>>
            ()
        ),)*
    >;
    type AggregateComponentState = (
        $(
            $Type::AggregateComponentState,
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
            EmptyElementData<ExplicitState>
        ),)*
        ExplicitState,
    >;

    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState) {
        let mut state : Self::AggregateComponentState = Default::default();

        let node = ElementTupleData(
            $(
                {
                    let (subnode, substate) = self.$index.build(prev_state.$index);
                    state.$index = substate;
                    subnode
                },
            )*
            $(replace_expr!(($Remainder) >>>
                EmptyElementData(Default::default())
            ),)*
            Default::default()
        );

        (node, state)
    }
}

    };
}

declare_stuff!{
    MyElementTuple_1;
    T0; __, __, __, __, __, __, __, __, __, __, __ ;
    0
}

declare_stuff!{
    MyElementTuple_2;
    T0, T1; __, __, __, __, __, __, __, __, __, __ ;
    0, 1
}

declare_stuff!{
    MyElementTuple_3;
    T0, T1, T2; __, __, __, __, __, __, __, __, __ ;
    0, 1, 2
}

declare_stuff!{
    MyElementTuple_4;
    T0, T1, T2, T3; __, __, __, __, __, __, __, __ ;
    0, 1, 2, 3
}

declare_stuff!{
    MyElementTuple_5;
    T0, T1, T2, T3, T4; __, __, __, __, __, __, __ ;
    0, 1, 2, 3, 4
}

declare_stuff!{
    MyElementTuple_6;
    T0, T1, T2, T3, T4, T5; __, __, __, __, __, __ ;
    0, 1, 2, 3, 4, 5
}

declare_stuff!{
    MyElementTuple_7;
    T0, T1, T2, T3, T4, T5, T6; __, __, __, __, __ ;
    0, 1, 2, 3, 4, 5, 6
}

declare_stuff!{
    MyElementTuple_8;
    T0, T1, T2, T3, T4, T5, T6, T7; __, __, __, __ ;
    0, 1, 2, 3, 4, 5, 6, 7
}

declare_stuff!{
    MyElementTuple_9;
    T0, T1, T2, T3, T4, T5, T6, T7, T8; __, __, __ ;
    0, 1, 2, 3, 4, 5, 6, 7, 8
}

declare_stuff!{
    MyElementTuple_10;
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9; __, __ ;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9
}

declare_stuff!{
    MyElementTuple_11;
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10; __ ;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
}

declare_stuff!{
    MyElementTuple_12;
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11 ;;
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
}

#[macro_export]
macro_rules! make_group {

    ( $(,)? ) => {
        $crate::elements::EmptyElement::new()
    };

    ( $e0:expr $(,)? ) => {
        $crate::elements::MyElementTuple_1(
            $e0,
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr $(,)? ) => {
        $crate::elements::MyElementTuple_2(
            $e0, $e1,
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr $(,)? ) => {
        $crate::elements::MyElementTuple_3(
            $e0, $e1, $e2,
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr $(,)? ) => {
        $crate::elements::MyElementTuple_4(
            $e0, $e1, $e2, $e3,
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr $(,)? ) => {
        $crate::elements::MyElementTuple_5(
            $e0, $e1, $e2, $e3, $e4,
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr $(,)? ) => {
        $crate::elements::MyElementTuple_6(
            $e0, $e1, $e2, $e3, $e4, $e5,
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr $(,)? ) => {
        $crate::elements::MyElementTuple_7(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6,
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr $(,)? ) => {
        $crate::elements::MyElementTuple_8(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7,
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr $(,)? ) => {
        $crate::elements::MyElementTuple_9(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8,
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr, $e9:expr $(,)? ) => {
        $crate::elements::MyElementTuple_10(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8, $e9,
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr, $e9:expr, $e10:expr $(,)? ) => {
        $crate::elements::MyElementTuple_11(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8, $e9, $e10
            Default::default()
        )
    };
    ( $e0:expr, $e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr, $e9:expr, $e10:expr, $e11:expr $(,)? ) => {
        $crate::elements::MyElementTuple_12(
            $e0, $e1, $e2, $e3, $e4, $e5, $e6, $e7, $e8, $e9, $e10, $e11
            Default::default()
        )
    };

}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EventEnum<
T0 = (),
T1 = (),
T2 = (),
T3 = (),
T4 = (),
T5 = (),
T6 = (),
T7 = (),
T8 = (),
T9 = (),
T10 = (),
T11 = (),
> {
    E0(T0),
    E1(T1),
    E2(T2),
    E3(T3),
    E4(T4),
    E5(T5),
    E6(T6),
    E7(T7),
    E8(T8),
    E9(T9),
    E10(T10),
    E11(T11),
}

impl<
        C0: VirtualDom<ParentComponentState>,
        C1: VirtualDom<ParentComponentState>,
        C2: VirtualDom<ParentComponentState>,
        C3: VirtualDom<ParentComponentState>,
        C4: VirtualDom<ParentComponentState>,
        C5: VirtualDom<ParentComponentState>,
        C6: VirtualDom<ParentComponentState>,
        C7: VirtualDom<ParentComponentState>,
        C8: VirtualDom<ParentComponentState>,
        C9: VirtualDom<ParentComponentState>,
        C10: VirtualDom<ParentComponentState>,
        C11: VirtualDom<ParentComponentState>,
        ParentComponentState,
    > VirtualDom<ParentComponentState> for ElementTupleData<
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
    ParentComponentState>
{
    type Event = EventEnum<
    C0::Event,
    C1::Event,
    C2::Event,
    C3::Event,
    C4::Event,
    C5::Event,
    C6::Event,
    C7::Event,
    C8::Event,
    C9::Event,
    C10::Event,
    C11::Event,
    >;
    type DomState = (
        C0::DomState,
        C1::DomState,
        C2::DomState,
        C3::DomState,
        C4::DomState,
        C5::DomState,
        C6::DomState,
        C7::DomState,
        C8::DomState,
        C9::DomState,
        C10::DomState,
        C11::DomState,
    );
    type AggregateComponentState = (
        C0::AggregateComponentState,
        C1::AggregateComponentState,
        C2::AggregateComponentState,
        C3::AggregateComponentState,
        C4::AggregateComponentState,
        C5::AggregateComponentState,
        C6::AggregateComponentState,
        C7::AggregateComponentState,
        C8::AggregateComponentState,
        C9::AggregateComponentState,
        C10::AggregateComponentState,
        C11::AggregateComponentState,
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

    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    fn init_tree(&self) -> (Self::TargetWidgetSeq, Self::DomState) {
        let (w0, s0) = self.0.init_tree();
        let (w1, s1) = self.1.init_tree();
        let (w2, s2) = self.2.init_tree();
        let (w3, s3) = self.3.init_tree();
        let (w4, s4) = self.4.init_tree();
        let (w5, s5) = self.5.init_tree();
        let (w6, s6) = self.6.init_tree();
        let (w7, s7) = self.7.init_tree();
        let (w8, s8) = self.8.init_tree();
        let (w9, s9) = self.9.init_tree();
        let (w10, s10) = self.10.init_tree();
        let (w11, s11) = self.11.init_tree();

        let widget = WidgetTuple(
            w0,
            w1,
            w2,
            w3,
            w4,
            w5,
            w6,
            w7,
            w8,
            w9,
            w10,
            w11,
        );
        let state = (
            s0,
            s1,
            s2,
            s3,
            s4,
            s5,
            s6,
            s7,
            s8,
            s9,
            s10,
            s11,
        );

        (widget, state)
    }

    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Self::DomState,
        widget: &mut Self::TargetWidgetSeq,
    ) -> Self::DomState {
        (
            self.0.apply_diff(&other.0, prev_state.0, &mut widget.0),
            self.1.apply_diff(&other.1, prev_state.1, &mut widget.1),
            self.2.apply_diff(&other.2, prev_state.2, &mut widget.2),
            self.3.apply_diff(&other.3, prev_state.3, &mut widget.3),
            self.4.apply_diff(&other.4, prev_state.4, &mut widget.4),
            self.5.apply_diff(&other.5, prev_state.5, &mut widget.5),
            self.6.apply_diff(&other.6, prev_state.6, &mut widget.6),
            self.7.apply_diff(&other.7, prev_state.7, &mut widget.7),
            self.8.apply_diff(&other.8, prev_state.8, &mut widget.8),
            self.9.apply_diff(&other.9, prev_state.9, &mut widget.9),
            self.10.apply_diff(&other.10, prev_state.10, &mut widget.10),
            self.11.apply_diff(&other.11, prev_state.11, &mut widget.11),
        )
    }

    fn process_event(
        &self,
        explicit_state: &mut ParentComponentState,
        children_state: &mut Self::AggregateComponentState,
        dom_state: &mut Self::DomState,
        _cx: &mut GlobalEventCx,
    ) -> Option<Self::Event> {
        let event0 = self
            .0
            .process_event(explicit_state, &mut children_state.0, &mut dom_state.0, _cx)
            .map(|event| EventEnum::E0(event));
        let event1 = self
            .1
            .process_event(explicit_state, &mut children_state.1, &mut dom_state.1, _cx)
            .map(|event| EventEnum::E1(event));
        let event2 = self
            .2
            .process_event(explicit_state, &mut children_state.2, &mut dom_state.2, _cx)
            .map(|event| EventEnum::E2(event));
        let event3 = self
            .3
            .process_event(explicit_state, &mut children_state.3, &mut dom_state.3, _cx)
            .map(|event| EventEnum::E3(event));
        let event4 = self
            .4
            .process_event(explicit_state, &mut children_state.4, &mut dom_state.4, _cx)
            .map(|event| EventEnum::E4(event));
        let event5 = self
            .5
            .process_event(explicit_state, &mut children_state.5, &mut dom_state.5, _cx)
            .map(|event| EventEnum::E5(event));
        let event6 = self
            .6
            .process_event(explicit_state, &mut children_state.6, &mut dom_state.6, _cx)
            .map(|event| EventEnum::E6(event));
        let event7 = self
            .7
            .process_event(explicit_state, &mut children_state.7, &mut dom_state.7, _cx)
            .map(|event| EventEnum::E7(event));
        let event8 = self
            .8
            .process_event(explicit_state, &mut children_state.8, &mut dom_state.8, _cx)
            .map(|event| EventEnum::E8(event));
        let event9 = self
            .9
            .process_event(explicit_state, &mut children_state.9, &mut dom_state.9, _cx)
            .map(|event| EventEnum::E9(event));
        let event10 = self
            .10
            .process_event(explicit_state, &mut children_state.10, &mut dom_state.10, _cx)
            .map(|event| EventEnum::E10(event));
        let event11 = self
            .11.process_event(explicit_state, &mut children_state.11, &mut dom_state.11, _cx)
            .map(|event| EventEnum::E11(event));

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
