use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::elements::compute_diff::compute_diff;
use crate::glue::GlobalEventCx;
use crate::widgets::WidgetList;

use crate::element_tree::ProcessEventCtx;
use crate::element_tree::ReconcileCtx;

use derivative::Derivative;
use either::{Left, Right};
use std::collections::VecDeque;
use tracing::{debug_span, info, instrument};

// TODO - Add arbitrary index types

/// A list of elements of the same type.
///
/// ## Events
///
/// Doesn't emit events.
///
/// ## About keys
///
/// ElementList stores a Vec of `(String, Child)`, where the string must be a unique key.
///
/// Keys help Panoramix figure out element identity, and maintain widget persistence. For instance, if your element list before update looks like:
///
///``` text
///[
///    ("foo-1", MyComponent("foo")),
///    ("foo-2", MyComponent("foo")),
///    ("bar-3", MyComponent("bar")),
///];
///```
///
/// and your element list post-update looks like:
///
///``` text
///[
///    ("foo-1", MyComponent("foo")),
///    ("bar-3", MyComponent("bar")),
///];
///```
///
/// Panoramix will figure out that the element at `foo-2` has been removed, and will remove it from the underlying widget tree, as well as perform any necessary cleanup.
#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = "Child: Clone"))]
pub struct ElementList<Child: Element<CpEvent, CpState>, CpEvent = NoEvent, CpState = ()> {
    pub children: Vec<(String, Child)>,
    pub _comp_state: std::marker::PhantomData<CpState>,
    pub _comp_event: std::marker::PhantomData<CpEvent>,
}

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = "Child: Clone"))]
pub struct ElementListData<Child: VirtualDom<CpEvent, CpState>, CpEvent = NoEvent, CpState = ()> {
    pub children: Vec<(String, Child)>,
    pub _comp_state: std::marker::PhantomData<CpState>,
    pub _comp_event: std::marker::PhantomData<CpEvent>,
}

// ----

impl<CpEvent, CpState, Child: Element<CpEvent, CpState>> ElementList<Child, CpEvent, CpState> {
    /// Build a list by providing an iterator of `(Key, Element)` pairs.
    pub fn from_pairs(pairs: impl std::iter::IntoIterator<Item = (String, Child)>) -> Self {
        Self {
            children: pairs.into_iter().collect(),
            _comp_state: Default::default(),
            _comp_event: Default::default(),
        }
    }

    /// Build a list by providing keys and elements as separate iterators.
    pub fn from_keys_elems(
        keys: impl std::iter::IntoIterator<Item = String>,
        elems: impl std::iter::IntoIterator<Item = Child>,
    ) -> Self {
        Self {
            children: keys.into_iter().zip(elems.into_iter()).collect(),
            _comp_state: Default::default(),
            _comp_event: Default::default(),
        }
    }
}

// ----

impl<CpEvent, CpState, Child: Element<CpEvent, CpState>> Element<CpEvent, CpState>
    for ElementList<Child, CpEvent, CpState>
{
    type Event = NoEvent;
    type ComponentState = crate::element_tree::NoState;
    type AggregateChildrenState = Vec<(String, Child::AggregateChildrenState)>;
    type BuildOutput = ElementListData<Child::BuildOutput, CpEvent, CpState>;

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

impl<CpEvent, CpState, Child: VirtualDom<CpEvent, CpState>> VirtualDom<CpEvent, CpState>
    for ElementListData<Child, CpEvent, CpState>
{
    type Event = NoEvent;
    type AggregateChildrenState = Vec<(String, Child::AggregateChildrenState)>;
    type TargetWidgetSeq = WidgetList<Child::TargetWidgetSeq>;

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

    #[instrument(name = "List", skip(self, comp_ctx, children_state, widget_seq, cx))]
    fn process_event(
        &self,
        comp_ctx: &mut ProcessEventCtx<CpEvent, CpState>,
        children_state: &mut Self::AggregateChildrenState,
        widget_seq: &mut Self::TargetWidgetSeq,
        cx: &mut GlobalEventCx,
    ) {
        for child_data in self
            .children
            .iter()
            .zip(children_state)
            .zip(widget_seq.children.iter_mut())
        {
            let (_key, child) = child_data.0 .0;
            let child_comp_state = child_data.0 .1;
            let child_widget_seq = child_data.1;
            child.process_event(comp_ctx, &mut child_comp_state.1, child_widget_seq, cx);
        }
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

    #[test]
    fn list_label_widget() {
        use crate::test_harness::Harness;
        let list = new_label_list(&["aaa", "bbb", "ccc"]);

        Harness::run_test_window(list.clone(), |harness| {
            let list_state = harness.get_root_debug_state();
            assert_debug_snapshot!(list_state);

            let mut new_list = list.clone();
            new_list.children[0].1 = Label::new("AAA");
            new_list
                .children
                .insert(1, (String::from("ddd"), Label::new("DDD")));
            harness.update_root_element(new_list);

            let list_state_2 = harness.get_root_debug_state();
            assert_debug_snapshot!(list_state_2);

            let empty_list = new_label_list(&[]);
            harness.update_root_element(empty_list);

            let empty_list_state = harness.get_root_debug_state();
            assert_debug_snapshot!(empty_list_state);
        });
    }
}
