use std::fmt::Debug;
use tracing::{instrument, trace};

// TODO - Reduce allocations here
#[derive(Default, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ListMutationItem<Key> {
    pub index: usize,
    pub preserved_count_before: usize,
    pub removed_count: usize,
    pub inserted_keys: Vec<Key>,
}

#[derive(Default, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ListMutation<Key> {
    pub items: Vec<ListMutationItem<Key>>,
}

impl<Key> ListMutation<Key> {
    pub fn new(items: Vec<ListMutationItem<Key>>) -> Self {
        ListMutation { items }
    }
}

#[instrument(skip(old_list, new_list))]
fn find_next_common_item<Key: Eq + Debug, T1, T2>(
    old_list: &[(Key, T1)],
    new_list: &[(Key, T2)],
    cursor_old: usize,
    cursor_new: usize,
) -> Option<(usize, usize)> {
    for idx_old in cursor_old..old_list.len() {
        for idx_new in cursor_new..new_list.len() {
            if old_list[idx_old].0 == new_list[idx_new].0 {
                return Some((idx_old, idx_new));
            }
        }
    }
    None
}

#[instrument(skip(old_list, new_list))]
pub fn compute_diff<Key: Eq + Debug + Clone, T1, T2>(
    old_list: &[(Key, T1)],
    new_list: &[(Key, T2)],
) -> ListMutation<Key> {
    let mut list_mutation = ListMutation { items: vec![] };
    let mut cursor_old = 0usize;
    let mut cursor_new = 0usize;
    let mut preserved_count_before = 0;

    // TODO - trace keys
    trace!("Comparing arrays");

    while let Some((next_old, next_new)) =
        find_next_common_item(old_list, new_list, cursor_old, cursor_new)
    {
        // If we skipped over some items, then we add then to a new mutation
        if cursor_new != next_new || cursor_old != next_old {
            list_mutation.items.push(ListMutationItem {
                index: cursor_old,
                preserved_count_before,
                removed_count: next_old - cursor_old,
                inserted_keys: new_list[cursor_new..next_new]
                    .iter()
                    .map(|item| item.0.clone())
                    .collect(),
            });
            preserved_count_before = 1;
        } else {
            preserved_count_before += 1;
        }
        cursor_old = next_old + 1;
        cursor_new = next_new + 1;
    }

    list_mutation.items.push(ListMutationItem {
        index: cursor_old,
        preserved_count_before,
        removed_count: old_list.len() - cursor_old,
        inserted_keys: new_list[cursor_new..new_list.len()]
            .iter()
            .map(|item| item.0.clone())
            .collect(),
    });

    list_mutation
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    fn make_list(keys: &[i32]) -> Vec<(i32, ())> {
        keys.iter().map(|key| (*key, ())).collect()
    }

    fn make_mut_tail(index: usize, preserved_count_before: usize) -> ListMutationItem<i32> {
        ListMutationItem {
            index,
            preserved_count_before,
            removed_count: 0,
            inserted_keys: vec![],
        }
    }

    // TODO - O(N * M) for N the list size and M the mutation count
    // Use better algo
    // TODO - Implement more generic version instead of copy-pasting everywhere
    fn apply_diff(old_list: &[(i32, ())], mutation: &ListMutation<i32>) -> Vec<(i32, ())> {
        let mut index_diff = 0_isize;
        let mut new_list: Vec<_> = old_list.into();

        // TODO - reserve

        for mutation_item in &mutation.items {
            let index = (mutation_item.index as isize + index_diff) as usize;
            let range = index..(index + mutation_item.removed_count);

            // Calling .last() runs the entire iterator
            let _ = new_list
                .splice(
                    range,
                    mutation_item
                        .inserted_keys
                        .iter()
                        .cloned()
                        .map(|key| (key, ())),
                )
                .last();

            index_diff += mutation_item.inserted_keys.len() as isize;
            index_diff -= mutation_item.removed_count as isize;
        }

        new_list
    }

    #[test]
    fn empty_lists() {
        let old_list = make_list(&[]);
        let new_list = make_list(&[]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![make_mut_tail(0, 0),]
            }
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn identical_lists() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[1, 2, 3]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![make_mut_tail(old_list.len(), 3),]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn inserted_one() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[1, 2, 999, 3]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![
                    ListMutationItem {
                        index: 2,
                        preserved_count_before: 2,
                        removed_count: 0,
                        inserted_keys: vec![999],
                    },
                    make_mut_tail(old_list.len(), 1),
                ]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn inserted_one_beginning() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[999, 1, 2, 3]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![
                    ListMutationItem {
                        index: 0,
                        preserved_count_before: 0,
                        removed_count: 0,
                        inserted_keys: vec![999],
                    },
                    make_mut_tail(old_list.len(), 3),
                ]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn inserted_one_end() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[1, 2, 3, 999]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![ListMutationItem {
                    index: 3,
                    preserved_count_before: 3,
                    removed_count: 0,
                    inserted_keys: vec![999],
                },]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn removed_one() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[1, 3]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![
                    ListMutationItem {
                        index: 1,
                        preserved_count_before: 1,
                        removed_count: 1,
                        inserted_keys: vec![],
                    },
                    make_mut_tail(old_list.len(), 1),
                ]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn removed_first() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[2, 3]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![
                    ListMutationItem {
                        index: 0,
                        preserved_count_before: 0,
                        removed_count: 1,
                        inserted_keys: vec![],
                    },
                    make_mut_tail(old_list.len(), 2),
                ]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn removed_last() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[1, 2]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![ListMutationItem {
                    index: 2,
                    preserved_count_before: 2,
                    removed_count: 1,
                    inserted_keys: vec![],
                },]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn removed_all() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![ListMutationItem {
                    index: 0,
                    preserved_count_before: 0,
                    removed_count: 3,
                    inserted_keys: vec![],
                },]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn changed_one() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[1, 999, 3]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![
                    ListMutationItem {
                        index: 1,
                        preserved_count_before: 1,
                        removed_count: 1,
                        inserted_keys: vec![999],
                    },
                    make_mut_tail(old_list.len(), 1),
                ]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn changed_first() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[999, 2, 3]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![
                    ListMutationItem {
                        index: 0,
                        preserved_count_before: 0,
                        removed_count: 1,
                        inserted_keys: vec![999],
                    },
                    make_mut_tail(old_list.len(), 2),
                ]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn changed_last() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[1, 2, 999]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![ListMutationItem {
                    index: 2,
                    preserved_count_before: 2,
                    removed_count: 1,
                    inserted_keys: vec![999],
                },]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn changed_all() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[101, 102, 103]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![ListMutationItem {
                    index: 0,
                    preserved_count_before: 0,
                    removed_count: 3,
                    inserted_keys: vec![101, 102, 103],
                },]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    #[test]
    pub fn swap_order() {
        let old_list = make_list(&[1, 2, 3]);
        let new_list = make_list(&[3, 2, 1]);

        let diff = compute_diff(&old_list, &new_list);
        assert_eq!(
            diff,
            ListMutation {
                items: vec![
                    ListMutationItem {
                        index: 0,
                        preserved_count_before: 0,
                        removed_count: 0,
                        inserted_keys: vec![3, 2],
                    },
                    ListMutationItem {
                        index: 1,
                        preserved_count_before: 1,
                        removed_count: 2,
                        inserted_keys: vec![],
                    },
                ]
            },
        );
        assert_eq!(apply_diff(&old_list, &diff), new_list);
    }

    // TODO
    // - Add more robust reconciliation tests with fuzzing
    // - Add a "multiple changes" test
}
