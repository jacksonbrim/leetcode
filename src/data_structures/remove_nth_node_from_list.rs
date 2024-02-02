use crate::linked_list_utils::*;
// Definition for singly-linked list.
struct Solution;
impl Solution {
    // 1ms, 69.35%, 2.05MB 70.16%
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;
        {
            let mut current = dummy.as_ref();
            while let Some(node) = current {
                len += 1;
                current = node.next.as_ref();
            }
        }

        let idx_to_remove = len - n - 1; // -1 to point to the node just before the one to be removed
        let mut current = dummy.as_mut();

        for _ in 0..idx_to_remove {
            current = current.unwrap().next.as_mut();
        }

        let next_next = current.as_mut().unwrap().next.as_mut().unwrap().next.take();
        current.as_mut().unwrap().next = next_next;

        dummy.unwrap().next
    }
    pub fn original_remove_nth_from_end(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        // adjust for the added dummy_head length.
        let mut length = -1;
        let mut current = dummy_head.as_ref();

        // Calculate the length of the list
        while let Some(node) = current {
            length += 1;
            current = node.next.as_ref();
        }
        if n == length && length == 1 {
            return None;
        }

        // If n is equal to the length of the list, remove the head
        if n == length {
            return dummy_head.unwrap().next.unwrap().next;
        }

        // Find the node before the one to remove
        let idx_to_remove = length - n;
        let mut current_mut = dummy_head.as_mut();

        for _ in 0..idx_to_remove {
            current_mut = current_mut.unwrap().next.as_mut();
        }

        // Skip the nth node from the end
        let next_next = current_mut
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .take();
        current_mut.as_mut().unwrap().next = next_next;

        dummy_head.unwrap().next
    }
    pub fn popular_on_leetcode_remove_nth_from_end(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        remove_nth_from_end_recr(head, n).0
    }
}

fn remove_nth_from_end_recr(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, usize) {
    match head {
        None => (None, 1),
        Some(mut node) => {
            let (prev, num) = remove_nth_from_end_recr(node.next.take(), n);
            if n == num as i32 {
                (prev, num + 1)
            } else {
                node.next = prev;
                (Some(node), num + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked_list_utils::*;

    #[test]
    fn from_vec_success() {
        let input = vec![1, 2, 3];
        let output: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let mut result = Some(Box::new(ListNode::from_vec(&input)));
        assert_eq!(result, output);
    }
    #[test]
    fn reversed_success() {
        let input = vec![1, 2, 3];
        let output: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));

        let mut result = Some(Box::new(ListNode::from_vec(&input)));
        result = Some(Box::new(result.unwrap().reverse()));
        assert_eq!(result, output);
    }

    #[test]
    fn remove_nth_from_end_success() {
        let input = vec![1, 2, 3, 4, 5];
        let input = Some(Box::new(ListNode::from_vec(&input)));
        let n = 2;
        let output = vec![1, 2, 3, 5];
        let output: Option<Box<ListNode>> = Some(Box::new(ListNode::from_vec(&output)));
        let mut result = Solution::remove_nth_from_end(input, n);

        assert_eq!(result, output);
    }
    #[test]
    fn remove_nth_from_end_one_node_success() {
        let input = vec![1];
        let input = Some(Box::new(ListNode::from_vec(&input)));
        let n = 1;
        let output = None;
        let mut result = Solution::remove_nth_from_end(input, n);

        assert_eq!(result, output);
    }
    #[test]
    fn remove_nth_from_end_two_nodes_skip_first_success() {
        let input = vec![1, 2];
        let input = Some(Box::new(ListNode::from_vec(&input)));
        let n = 2;
        let output = vec![2];
        let output: Option<Box<ListNode>> = Some(Box::new(ListNode::from_vec(&output)));
        let mut result = Solution::remove_nth_from_end(input, n);

        assert_eq!(result, output);
    }
}
