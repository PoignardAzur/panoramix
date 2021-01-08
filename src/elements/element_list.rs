use either::{Left, Right};
use std::collections::VecDeque;

use crate::element_tree::{ElementTree, VirtualDom};
use crate::elements::compute_diff::compute_diff;
use crate::glue::GlobalEventCx;
use crate::widgets::WidgetList;

// TODO - Add arbitrary index types

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ElementList<Child: ElementTree<ExplicitState>, ExplicitState = ()> {
    pub children: Vec<(String, Child)>,
    pub _expl_state: std::marker::PhantomData<ExplicitState>,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ElementListData<Item: VirtualDom<ParentComponentState>, ParentComponentState> {
    pub children: Vec<(String, Item)>,
    pub _expl_state: std::marker::PhantomData<ParentComponentState>,
}

// ----

impl<ExplicitState, Child: ElementTree<ExplicitState>> ElementTree<ExplicitState>
    for ElementList<Child, ExplicitState>
{
    type Event = (usize, Child::Event);
    type AggregateComponentState = Vec<(String, Child::AggregateComponentState)>;
    type BuildOutput = ElementListData<Child::BuildOutput, ExplicitState>;

    fn build(
        self,
        prev_state: Self::AggregateComponentState,
    ) -> (Self::BuildOutput, Self::AggregateComponentState) {
        // FIXE - Handle duplicate keys
        // TODO - Add special case when Child::AggregateComponentState.sizeof() == 0

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
                _expl_state: Default::default(),
            },
            new_state,
        )
    }
}

impl<Item: VirtualDom<ParentComponentState>, ParentComponentState> VirtualDom<ParentComponentState>
    for ElementListData<Item, ParentComponentState>
{
    type Event = (usize, Item::Event);
    type DomState = Vec<Item::DomState>;
    type AggregateComponentState = Vec<(String, Item::AggregateComponentState)>;

    type TargetWidgetSeq = WidgetList<Item::TargetWidgetSeq>;

    fn update_value(&mut self, other: Self) {
        *self = other;
    }

    fn init_tree(&self) -> (Self::TargetWidgetSeq, Self::DomState) {
        let (widgets, dom_state): (Vec<_>, Vec<_>) = self
            .children
            .iter()
            .map(|(_, elem)| elem.init_tree())
            .unzip();

        (WidgetList { children: widgets }, dom_state)
    }

    // FIXME
    // This only works if we assume that items are ever only added at the end of the list.
    // Sounds perfectly reasonable to me.
    // (seriously though, a serious implementation would try to do whatever crochet::List::run does)
    fn apply_diff(
        &self,
        other: &Self,
        prev_state: Self::DomState,
        widget: &mut Self::TargetWidgetSeq,
    ) -> Self::DomState {
        let mutation = compute_diff(&other.children, &self.children);

        let mut prev_data: Vec<_> = other
            .children
            .iter()
            .zip(prev_state)
            .zip(widget.children.iter_mut())
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
        let mut updated_state: Vec<_> = self
            .children
            .iter()
            .zip(prev_data)
            .map(|item| {
                let (key, child_data) = item.0;
                let child_prev_data = item.1;
                match child_prev_data {
                    Left(prev_data) => {
                        let (((_key, child_prev_data), child_prev_state), child_widget) = prev_data;
                        child_data.apply_diff(child_prev_data, child_prev_state, child_widget)
                    }
                    Right(new_data) => {
                        let (_key, child_data) = new_data;
                        let (new_widget_seq, new_state) = child_data.init_tree();
                        widgets_to_insert.push_back(new_widget_seq);
                        new_state
                    }
                }
            })
            .collect();

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
            let _ = widget
                .children
                .splice(
                    spliced_range,
                    widgets_to_insert.drain(0..mutation_item.inserted_keys.len()),
                )
                .last();

            index_diff += mutation_item.inserted_keys.len() as isize;
            index_diff -= mutation_item.removed_count as isize;
        }

        updated_state
    }

    fn process_event(
        &self,
        explicit_state: &mut ParentComponentState,
        children_state: &mut Self::AggregateComponentState,
        dom_state: &mut Self::DomState,
        _cx: &mut GlobalEventCx,
    ) -> Option<(usize, Item::Event)> {
        for (i, elem_data) in self
            .children
            .iter()
            .zip(children_state)
            .zip(dom_state)
            .enumerate()
        {
            let (_key, element) = elem_data.0 .0;
            let elem_comp_state = elem_data.0 .1;
            let elem_dom_state = elem_data.1;
            if let Some(event) =
                element.process_event(explicit_state, &mut elem_comp_state.1, elem_dom_state, _cx)
            {
                return Some((i, event));
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::label::{Label, LabelData};
    use crate::elements::{MockState, WithMockState};

    fn new_label_list<State>(names: &[&str]) -> ElementList<Label<State>, State> {
        let children: Vec<_> = names
            .into_iter()
            .map(|name| (String::from(*name), Label::new(*name)))
            .collect();
        ElementList {
            children,
            _expl_state: Default::default(),
        }
    }

    #[test]
    fn new_list() {
        let list = new_label_list::<()>(&["aaa", "bbb", "ccc"]);
        let (list_data, _) = list.clone().build(Default::default());

        assert_eq!(
            list,
            ElementList::<_, ()> {
                children: vec![
                    (String::from("aaa"), Label::new("aaa")),
                    (String::from("bbb"), Label::new("bbb")),
                    (String::from("ccc"), Label::new("ccc")),
                ],
                _expl_state: Default::default(),
            },
        );
        assert_eq!(
            list_data,
            ElementListData {
                children: vec![
                    (String::from("aaa"), LabelData::new("aaa")),
                    (String::from("bbb"), LabelData::new("bbb")),
                    (String::from("ccc"), LabelData::new("ccc")),
                ],
                _expl_state: Default::default()
            },
        );
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
            _expl_state: Default::default(),
        }
    }

    #[test]
    fn new_list_with_existing_state() {
        let list = new_label_list_with_state::<MockState>(&["aaa", "bbb", "ccc"]);
        let list_state = vec![
            (String::from("bbb"), (MockState::new("Foobar"), ())),
            (
                String::from("notfound"),
                (MockState::new("ThisShouldBeDropped"), ()),
            ),
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
