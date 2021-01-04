use crate::glue::GlobalEventCx;
use crate::widgets::WidgetTuple;

use crate::element_tree::{ElementTree, VirtualDom};

pub struct ElementTuple<
    E0: ElementTree<ExplicitState>,
    E1: ElementTree<ExplicitState>,
    E2: ElementTree<ExplicitState>,
    E3: ElementTree<ExplicitState>,
    ExplicitState = (),
>(
    pub E0,
    pub E1,
    pub E2,
    pub E3,
    pub std::marker::PhantomData<ExplicitState>,
);

pub struct ElementTupleTarget<
    C0: VirtualDom<ParentComponentState>,
    C1: VirtualDom<ParentComponentState>,
    C2: VirtualDom<ParentComponentState>,
    C3: VirtualDom<ParentComponentState>,
    ParentComponentState,
>(
    pub C0,
    pub C1,
    pub C2,
    pub C3,
    pub std::marker::PhantomData<ParentComponentState>,
);

impl<
        ExplicitState,
        E0: ElementTree<ExplicitState>,
        E1: ElementTree<ExplicitState>,
        E2: ElementTree<ExplicitState>,
        E3: ElementTree<ExplicitState>,
    > ElementTree<ExplicitState> for ElementTuple<E0, E1, E2, E3, ExplicitState>
{
    type Event = EventEnum<E0::Event, E1::Event, E2::Event, E3::Event>;
    type AggregateComponentState = (
        E0::AggregateComponentState,
        E1::AggregateComponentState,
        E2::AggregateComponentState,
        E3::AggregateComponentState,
    );
    type BuildOutput = ElementTupleTarget<
        E0::BuildOutput,
        E1::BuildOutput,
        E2::BuildOutput,
        E3::BuildOutput,
        ExplicitState,
    >;

    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState) {
        let (t0, s0) = self.0.build(prev_state.0);
        let (t1, s1) = self.1.build(prev_state.1);
        let (t2, s2) = self.2.build(prev_state.2);
        let (t3, s3) = self.3.build(prev_state.3);

        (
            ElementTupleTarget(t0, t1, t2, t3, Default::default()),
            (s0, s1, s2, s3),
        )
    }
}

pub enum EventEnum<T0, T1, T2, T3> {
    E0(T0),
    E1(T1),
    E2(T2),
    E3(T3),
}

impl<
        C0: VirtualDom<ParentComponentState>,
        C1: VirtualDom<ParentComponentState>,
        C2: VirtualDom<ParentComponentState>,
        C3: VirtualDom<ParentComponentState>,
        ParentComponentState,
    > VirtualDom<ParentComponentState>
    for ElementTupleTarget<C0, C1, C2, C3, ParentComponentState>
{
    type Event = EventEnum<C0::Event, C1::Event, C2::Event, C3::Event>;
    type DomState = (C0::DomState, C1::DomState, C2::DomState, C3::DomState);
    type AggregateComponentState = (
        C0::AggregateComponentState,
        C1::AggregateComponentState,
        C2::AggregateComponentState,
        C3::AggregateComponentState,
    );

    type TargetWidgetSeq = WidgetTuple<
        C0::TargetWidgetSeq,
        C1::TargetWidgetSeq,
        C2::TargetWidgetSeq,
        C3::TargetWidgetSeq,
    >;

    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    fn init_tree(&self) -> (Self::TargetWidgetSeq, Self::DomState) {
        let (w0, s0) = self.0.init_tree();
        let (w1, s1) = self.1.init_tree();
        let (w2, s2) = self.2.init_tree();
        let (w3, s3) = self.3.init_tree();

        (WidgetTuple(w0, w1, w2, w3), (s0, s1, s2, s3))
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

        // FIXME - If several events happen simultaneously, this will swallow all but one
        // process_event should return an iterator or an observable instead.
        event0.or(event1).or(event2).or(event3)
    }
}
