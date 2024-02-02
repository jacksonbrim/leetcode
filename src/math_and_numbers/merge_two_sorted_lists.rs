use crate::linked_list_utils::*;
pub struct Solution;
impl Solution {
    // First intuition, 1ms runtims, beats 72.79% of Rust users, 2.08MB Memory beating 81.25%
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::dfs(&list1, &list2)
    }
    // depth first search
    fn dfs(list1: &Option<Box<ListNode>>, list2: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(node1), Some(node2)) if node1.val < node2.val => {
                let mut new_node = ListNode::new(node1.val);
                new_node.next = Self::dfs(&node1.next, &list2);
                Some(Box::new(new_node))
            }
            (Some(_node1), Some(node2)) => {
                let mut new_node = ListNode::new(node2.val);
                new_node.next = Self::dfs(&list1, &node2.next);
                Some(Box::new(new_node))
            }
            (Some(node1), None) => {
                let mut new_node = ListNode::new(node1.val);
                new_node.next = Self::dfs(&node1.next, &list2);
                Some(Box::new(new_node))
            }
            (None, Some(node2)) => {
                let mut new_node = ListNode::new(node2.val);
                new_node.next = Self::dfs(&list1, &node2.next);
                Some(Box::new(new_node))
            }
            _ => None, // Both lists are empty
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success() {
        let list1 = vec![1, 2, 4];
        let list1 = Some(Box::new(ListNode::from_vec(&list1)));
        let list2 = vec![1, 3, 4];
        let list2 = Some(Box::new(ListNode::from_vec(&list2)));
        let output = vec![1, 1, 2, 3, 4, 4];
        let output = Some(Box::new(ListNode::from_vec(&output)));
        let res = Solution::merge_two_lists(list1, list2);
        assert_eq!(res, output);
    }
    //#[test]
    //fn success() {
    //    let list1 =
    //    let list2 =
    //    let output =
    //    let res = SolutioN::merge_two_lists(list1, list2);
    //    assert_eq!(res, output);
    //}
    //#[test]
    //fn success() {
    //    let list1 =
    //    let list2 =
    //    let output =
    //    let res = SolutioN::merge_two_lists(list1, list2);
    //    assert_eq!(res, output);
    //}
}
