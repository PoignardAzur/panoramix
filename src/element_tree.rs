use crate::glue::GlobalEventCx;
use crate::widget_sequence::WidgetSequence;

pub trait ElementTree<ExplicitState> {
    type Event;
    type AggregateComponentState: Default;
    type BuildOutput: VirtualDom<
        ExplicitState,
        Event = Self::Event,
        AggregateComponentState = Self::AggregateComponentState,
    >;

    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState);
}

// TODO - Include documentation about what a Virtual DOM is and where the name comes from.
pub trait VirtualDom<ParentComponentState> {
    type Event;
    type AggregateComponentState: Default;

    // TODO - Might be superfluous
    type DomState;

    type TargetWidgetSeq: WidgetSequence;

    // update_value is intended to enable memoize-style HOC
    // where instead of returning a vdom node, it returns
    // something along the lines of struct KeepEverythingAsItWas()
    // Ugh. I'm not explaining this well.
    fn update_value(&mut self, other: Self);

    fn init_tree(&self) -> (Self::TargetWidgetSeq, Self::DomState);

    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Self::DomState,
        widget_seq: &mut Self::TargetWidgetSeq,
    ) -> Self::DomState;

    fn process_event(
        &self,
        explicit_state: &mut ParentComponentState,
        children_state: &mut Self::AggregateComponentState,
        dom_state: &mut Self::DomState,
        _cx: &mut GlobalEventCx,
    ) -> Option<Self::Event>;
}

/*
use crate::glue::{DruidAppData, GlobalEventCx, Id};

use crate::react_widgets::make_button;
use crate::react_widgets::{SingleWidget, WidgetList, WidgetSequence, WidgetTuple};

// TODO - refactor away WidgetPod
use druid::widget as druid_w;
use druid::WidgetPod;


use druid::kurbo::{Rect, Size};

use crate::flex2::FlexParams;
use crate::glue::DruidAppData;
use druid::widget::Button;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx,
    Widget, WidgetPod,
};
*/
