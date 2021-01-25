use crate::element_tree::{ElementTree, NoEvent, VirtualDom};
use crate::glue::{DruidAppData, GlobalEventCx};
use crate::widgets::SingleWidget;

use derivative::Derivative;
use druid::widget as druid_w;
use tracing::instrument;

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct Label<ComponentState = (), ComponentEvent = NoEvent>(
    pub String,
    pub std::marker::PhantomData<ComponentState>,
    pub std::marker::PhantomData<ComponentEvent>,
);

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct LabelData<ComponentState = (), ComponentEvent = NoEvent>(
    pub String,
    pub std::marker::PhantomData<ComponentState>,
    pub std::marker::PhantomData<ComponentEvent>,
);

//
// --- IMPLS

impl<ComponentState, ComponentEvent> Label<ComponentState, ComponentEvent> {
    pub fn new(text: impl Into<String>) -> Label<ComponentState, ComponentEvent> {
        Label(text.into(), Default::default(), Default::default())
    }

    pub fn with_mock_state(self) -> super::WithMockState<Self, ComponentState, ComponentEvent> {
        super::WithMockState::new(self)
    }
}

impl<ComponentState, ComponentEvent> LabelData<ComponentState, ComponentEvent> {
    pub fn new(text: impl Into<String>) -> LabelData<ComponentState, ComponentEvent> {
        LabelData(text.into(), Default::default(), Default::default())
    }

    pub fn with_mock_state(self) -> super::WithMockStateData<Self, ComponentState, ComponentEvent> {
        super::WithMockStateData::new(self)
    }
}

impl<ComponentState, ComponentEvent> ElementTree<ComponentState, ComponentEvent>
    for Label<ComponentState, ComponentEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type BuildOutput = LabelData<ComponentState, ComponentEvent>;

    #[instrument(name = "Label", skip(self, _prev_state))]
    fn build(self, _prev_state: ()) -> (LabelData<ComponentState, ComponentEvent>, ()) {
        (
            LabelData(self.0, Default::default(), Default::default()),
            (),
        )
    }
}

impl<ComponentState, ComponentEvent> VirtualDom<ComponentState, ComponentEvent>
    for LabelData<ComponentState, ComponentEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = ();
    type TargetWidgetSeq = SingleWidget<druid_w::Label<DruidAppData>>;

    #[instrument(name = "Label", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "Label", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        let text = &self.0;
        let label = druid_w::Label::new(text.clone());
        SingleWidget::new(label)
    }

    #[instrument(name = "Label", skip(self, other, widget))]
    fn reconcile(&self, other: &Self, widget: &mut Self::TargetWidgetSeq) {
        let text = &self.0;
        let prev_text = &other.0;
        if text != prev_text {
            widget.0.widget_mut().set_text(text.clone());
        }
    }

    #[instrument(name = "Label", skip(self, _component_state, _widget, _cx))]
    fn process_event(
        &self,
        _component_state: &mut ComponentState,
        _children_state: &mut (),
        _widget: &mut Self::TargetWidgetSeq,
        _cx: &mut GlobalEventCx,
    ) -> Option<NoEvent> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element_tree::assign_empty_state_type;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn new_label() {
        let label = Label::new("Hello");
        let (label_data, ()) = label.clone().build(());

        assert_debug_snapshot!(label);
        assert_debug_snapshot!(label_data);

        assert_eq!(label_data, LabelData::new("Hello"));

        assign_empty_state_type(&label);
    }

    // TODO
    // - Widget test
}
