use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::elements::compute_diff::compute_diff;
use crate::glue::GlobalEventCx;
use crate::widgets::WidgetList;

use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use either::{Left, Right};
use std::collections::VecDeque;
use tracing::{debug_span, info, instrument};

// TODO - Add arbitrary index types

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = "Child: Clone"))]
pub struct ElementList<Child: Element<CpState, CpEvent>, CpState = (), CpEvent = NoEvent> {
    pub children: Vec<(String, Child)>,
    pub _comp_state: std::marker::PhantomData<CpState>,
    pub _comp_event: std::marker::PhantomData<CpEvent>,
}

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = "Child: Clone"))]
pub struct ElementListData<Child: VirtualDom<CpState, CpEvent>, CpState = (), CpEvent = NoEvent> {
    pub children: Vec<(String, Child)>,
    pub _comp_state: std::marker::PhantomData<CpState>,
    pub _comp_event: std::marker::PhantomData<CpEvent>,
}

// ----

impl<CpState, CpEvent, Child: Element<CpState, CpEvent>> Element<CpState, CpEvent>
    for ElementList<Child, CpState, CpEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = Vec<(String, Child::AggregateChildrenState)>;
    type BuildOutput = ElementListData<Child::BuildOutput, CpState, CpEvent>;

    #[instrument(name = "List", skip(self, prev_state))]
    fn build(
        self,
        prev_state: Self::AggregateChildrenState,
    ) -> (Self::BuildOutput, Self::AggregateChildrenState) {
        // FIXE - Handle duplicate keys
        // TODO - Add special case when Child::AggregateChildrenState.sizeof() == 0

        let mutation = compute_diff(&prev_state, &self.children);
        let mut prev_state_or_default = prev_state;

        // TODO - reserve
        // TODO - O(N * M) for N the list size and M the mutation count
        // Use better algo
        let mut index_diff = 0_isize;
        for mutation_item in &mutation.items {
            let index = (mutation_item.index as isize + index_diff) as usize;
            let range = index..(index + mutation_item.removed_count);

            // Calling .last() runs the entire iterator, which performs the splice
            let _ = prev_state_or_default
                .splice(
                    range,
                    mutation_item
                        .inserted_keys
                        .iter()
                        .cloned()
                        .map(|key| (key, Default::default())),
                )
                .last();

            index_diff += mutation_item.inserted_keys.len() as isize;
            index_diff -= mutation_item.removed_count as isize;
        }

        let (children, new_state): (Vec<_>, Vec<_>) = self
            .children
            .into_iter()
            .zip(prev_state_or_default)
            .map(|((key, item), (_key, item_prev_state))| {
                let (new_item, new_state) = item.build(item_prev_state);
                ((key.clone(), new_item), (key, new_state))
            })
            .unzip();

        (
            ElementListData {
                children,
                ..Default::default()
            },
            new_state,
        )
    }
}

impl<CpState, CpEvent, Child: VirtualDom<CpState, CpEvent>> VirtualDom<CpState, CpEvent>
    for ElementListData<Child, CpState, CpEvent>
{
    type Event = NoEvent;
    type AggregateChildrenState = Vec<(String, Child::AggregateChildrenState)>;
    type TargetWidgetSeq = WidgetList<Child::TargetWidgetSeq>;

    #[instrument(name = "List", skip(self, other))]
    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    #[instrument(name = "List", skip(self))]
    fn init_tree(&self) -> Self::TargetWidgetSeq {
        WidgetList {
            children: self
                .children
                .iter()
                .map(|(_key, elem)| elem.init_tree())
                .collect(),
        }
    }

    #[instrument(name = "List", skip(self, other, widget_seq, ctx))]
    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        let mutation = compute_diff(&other.children, &self.children);

        let mut prev_data: Vec<_> = other
            .children
            .iter()
            .zip(widget_seq.children.iter_mut())
            .map(Left)
            .collect();

        // TODO - reserve
        // TODO - O(N * M) for N the list size and M the mutation count
        // Use better algo
        // TODO - simplify
        let mut index_diff = 0_isize;
        for mutation_item in &mutation.items {
            let index = (mutation_item.index as isize + index_diff) as usize;
            let spliced_range = index..(index + mutation_item.removed_count);

            let new_range = index..(index + mutation_item.inserted_keys.len());

            // Calling .last() runs the entire iterator, which performs the splice
            let _ = prev_data
                .splice(spliced_range, self.children[new_range].iter().map(Right))
                .last();

            index_diff += mutation_item.inserted_keys.len() as isize;
            index_diff -= mutation_item.removed_count as isize;
        }

        let mut widgets_to_insert = VecDeque::new();
        for item in self.children.iter().zip(prev_data) {
            let (_key, child_data) = item.0;
            let child_prev_data = item.1;
            match child_prev_data {
                Left(prev_data) => {
                    let ((_key, child_prev_data), child_widget_seq) = prev_data;
                    child_data.reconcile(child_prev_data, child_widget_seq, ctx);
                }
                Right(new_data) => {
                    let (_key, child_data) = new_data;
                    let new_widget_seq = debug_span!("init_tree").in_scope(|| {
                        info!("creating child");
                        child_data.init_tree()
                    });
                    widgets_to_insert.push_back(new_widget_seq);
                }
            }
        }

        // TODO - reserve
        // TODO - O(N * M) for N the list size and M the mutation count
        // Use better algo
        // TODO - simplify
        let mut index_diff = 0_isize;
        for mutation_item in &mutation.items {
            let index = (mutation_item.index as isize + index_diff) as usize;
            let spliced_range = index..(index + mutation_item.removed_count);

            // Calling .last() runs the entire iterator, which performs the splice
            //.insert(index, new_widget_seq);
            let _ = widget_seq
                .children
                .splice(
                    spliced_range,
                    widgets_to_insert.drain(0..mutation_item.inserted_keys.len()),
                )
                .last();

            index_diff += mutation_item.inserted_keys.len() as isize;
            index_diff -= mutation_item.removed_count as isize;
        }
    }

    #[instrument(
        name = "List",
        skip(self, component_state, children_state, widget_seq, cx)
    )]
    fn process_event(
        &self,
        component_state: &mut CpState,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) -> Option<CpEvent> {
        for child_data in self
            .children
            .iter()
            .zip(children_state)
            .zip(widget_seq.children.iter_mut())
        {
            let (_key, child) = child_data.0 .0;
            let child_comp_state = child_data.0 .1;
            let child_widget_seq = child_data.1;
            if let Some(event) = child.process_event(
                component_state,
                &mut child_comp_state.1,
                child_widget_seq,
                cx,
            ) {
                return Some(event);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element_tree::assign_empty_state_type;
    use crate::elements::label::{Label, LabelData};
    use crate::elements::{MockState, WithMockState};
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    fn new_label_list<State>(names: &[&str]) -> ElementList<Label<State>, State> {
        let children: Vec<_> = names
            .into_iter()
            .map(|name| (String::from(*name), Label::new(*name)))
            .collect();
        ElementList {
            children,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
        }
    }

    fn new_label_list_with_state<State>(
        names: &[&str],
    ) -> ElementList<WithMockState<Label<State>, State>, State> {
        let children: Vec<_> = names
            .iter()
            .map(|name| (String::from(*name), Label::new(*name).with_mock_state()))
            .collect();
        ElementList {
            children,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
        }
    }

    #[test]
    fn empty_list() {
        let list = new_label_list(&[]);
        let (list_data, _) = list.clone().build(Default::default());

        assert_debug_snapshot!(list);
        assert_debug_snapshot!(list_data);

        assign_empty_state_type(&list);
    }

    #[test]
    fn new_list() {
        let list = new_label_list(&["aaa", "bbb", "ccc"]);
        let (list_data, _) = list.clone().build(Default::default());

        assert_debug_snapshot!(list);
        assert_debug_snapshot!(list_data);

        assert_eq!(
            list_data,
            ElementListData {
                children: vec![
                    (String::from("aaa"), LabelData::new("aaa")),
                    (String::from("bbb"), LabelData::new("bbb")),
                    (String::from("ccc"), LabelData::new("ccc")),
                ],
                _comp_state: Default::default(),
                _comp_event: Default::default(),
            },
        );

        assign_empty_state_type(&list);
    }

    #[test]
    fn new_list_with_existing_state() {
        let list = new_label_list_with_state::<MockState>(&["aaa", "bbb", "ccc"]);
        let list_state = vec![
            (String::from("bbb"), (MockState::new("Foobar"), ())),
            (String::from("notfound"), (MockState::new("IAmError"), ())),
        ];
        let (_, new_list_state) = list.clone().build(list_state);

        assert_eq!(
            new_list_state,
            vec![
                (String::from("aaa"), (MockState::new("HelloWorld"), ())),
                (String::from("bbb"), (MockState::new("Foobar"), ())),
                (String::from("ccc"), (MockState::new("HelloWorld"), ())),
            ],
        );
    }

    // TODO
    // - Add constructor
    // - Widget test
    // - Event test
}
