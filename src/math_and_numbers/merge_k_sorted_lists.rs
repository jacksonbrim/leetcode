use crate::linked_list_utils::*;
use crate::math_and_numbers::merge_two_sorted_lists::Solution as MergeTwoSolution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;
impl Solution {
    // binary heap implementation
    // Reverse struct
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::with_capacity(lists.len());

        // Push Reverse objects into the heap
        for list in lists {
            if let Some(node) = list {
                heap.push(Reverse(node));
            }
        }

        let mut dummy_node = Box::new(ListNode::new(0));
        let mut curr_node = &mut dummy_node;

        while let Some(Reverse(node)) = heap.pop() {
            curr_node.next = Some(Box::new(ListNode::new(node.val)));
            curr_node = curr_node.next.as_mut().unwrap();

            if node.next.is_some() {
                heap.push(Reverse(node.next.unwrap()));
            }
        }

        dummy_node.next
    }

    // 89ms, beats 10.05%, 3.41MB beats 9.59% of users
    pub fn naive_merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        match lists.len() {
            0 => None,
            1 => lists.into_iter().next().unwrap_or(None),
            2 => {
                let mut lists_iter = lists.into_iter();
                let node1 = lists_iter.next().unwrap();
                let node2 = lists_iter.next().unwrap();
                return MergeTwoSolution::dfs(node1, node2);
            }
            _ => {
                // Handle the case where there are more than 2 lists
                // ...
                let mut lists = lists;
                Self::dfs_k(&mut lists)
            }
        }
        // get minimum two head nodes
        // merge into result list
        // repeat for all nodes
    }

    fn dfs_k(lists: &mut Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.iter().all(|list| list.is_none()) {
            return None;
        }

        // Find the index and node of the smallest value
        let mut min_index = None;
        let mut min_val = i32::MAX;
        for (i, list) in lists.iter().enumerate() {
            if let Some(node) = list {
                if node.val < min_val {
                    min_val = node.val;
                    min_index = Some(i);
                }
            }
        }

        // Advance the node at the found index
        if let Some(index) = min_index {
            let smallest_node = lists[index].take(); // Take the node out
            if let Some(mut node) = smallest_node {
                lists[index] = node.next.take(); // Advance the pointer in the list
                node.next = Self::dfs_k(lists); // Recursively merge the rest
                return Some(node);
            }
        }

        None
    }
    fn compare_list_nodes(
        lists: &Vec<Option<Box<ListNode>>>,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut least_node: Option<&Box<ListNode>> = lists[0].as_ref();
        let mut second_least_node: Option<&Box<ListNode>> = None;

        for list in lists.iter().skip(1) {
            if let Some(current_node) = list.as_ref() {
                match least_node {
                    Some(ln) if current_node.val < ln.val => {
                        second_least_node = least_node;
                        least_node = Some(current_node);
                    }
                    Some(_ln) => {
                        if second_least_node.is_none()
                            || current_node.val < second_least_node.unwrap().val
                        {
                            second_least_node = Some(current_node);
                        }
                    }
                    None => {
                        least_node = Some(current_node);
                    }
                }
            }
        }

        // Convert back to Option<Box<ListNode>> if needed
        (least_node.cloned(), second_least_node.cloned())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn naive_merge_k_linked_lists_success() {
        let list1 = vec![1, 4, 5];
        let list1 = Some(Box::new(ListNode::from_vec(&list1)));
        let list2 = vec![1, 3, 4];
        let list2 = Some(Box::new(ListNode::from_vec(&list2)));
        let list3 = vec![2, 6];
        let list3 = Some(Box::new(ListNode::from_vec(&list3)));

        let output = vec![1, 1, 2, 3, 4, 4, 5, 6];
        let output = Some(Box::new(ListNode::from_vec(&output)));
        let k_lists = vec![list1, list2, list3];
        let res = Solution::naive_merge_k_lists(k_lists);
        assert_eq!(res, output);
    }
    #[test]
    fn binary_heap_merge_k_linked_lists_success() {
        let list1 = vec![1, 4, 5];
        let list1 = Some(Box::new(ListNode::from_vec(&list1)));
        let list2 = vec![1, 3, 4];
        let list2 = Some(Box::new(ListNode::from_vec(&list2)));
        let list3 = vec![2, 6];
        let list3 = Some(Box::new(ListNode::from_vec(&list3)));

        let output = vec![1, 1, 2, 3, 4, 4, 5, 6];
        let output = Some(Box::new(ListNode::from_vec(&output)));
        let k_lists = vec![list1, list2, list3];
        let res = Solution::naive_merge_k_lists(k_lists);
        assert_eq!(res, output);
    }
}
