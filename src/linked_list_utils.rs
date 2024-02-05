use std::{cmp::Ordering, fmt};
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}
impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ {}", self.val)?;
        let mut current = &self.next;
        while let Some(node) = current {
            write!(f, " -> {}", node.val)?;
            current = &node.next;
        }
        write!(f, " -> None ]")
    }
}
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn push_val(mut self, val: i32) {
        let new_node = ListNode::new(val);
        let mut prev: Option<Box<ListNode>> = self.next;
        while prev.is_some() {
            if let Some(next) = prev {
                prev = Some(next);
            }
        }
        if let Some(mut node) = prev {
            node.next = Some(Box::new(new_node));
        }
    }

    pub fn push_node(mut self, new_node: Option<Box<ListNode>>) {
        let mut curr = self;
        while let Some(ref mut node) = curr.next {
            curr = node.as_ref().clone(); // Move ownership of the current node to avoid dangling pointers
        }
        curr.next = new_node;
    }

    pub fn from_vec(vals: &Vec<i32>) -> Self {
        if vals.len() > 1 {
            let node: ListNode = Self {
                val: vals[0],
                next: Some(Box::new(Self::from_vec(&vals[1..].to_vec()))),
            };
            return node;
        } else {
            return Self::new(vals[0]);
        }
    }
    // Reverses the list
    pub fn reverse(mut self) -> Self {
        let mut prev = None;
        let mut current = Some(Box::new(self));

        while let Some(mut node) = current {
            let next = node.next.take(); // Temporarily take ownership of next node
            node.next = prev; // Reverse the pointer
            prev = Some(node); // Move to next node
            current = next;
        }

        // Unwrap the reversed list and return
        *prev.unwrap()
    }
}

pub fn reverse_linked_list(list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = list;

    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        curr = next;
    }

    prev
}
pub fn create_list_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &val in vec.iter().rev() {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.next = current.take();
        *current = Some(new_node);
    }

    head
}
pub fn list_to_int(list: Option<Box<ListNode>>) -> i32 {
    let mut current = &list;
    let mut elements = Vec::new();

    while let Some(node) = current {
        elements.push(node.val.to_string());
        current = &node.next;
    }
    let digits: Vec<i32> = elements.iter().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut res: i32 = 0;
    for (i, &digit) in digits.iter().enumerate() {
        // Calculate the exponent part
        // The pow method requires a u32, so we cast i to u32
        let exponent = digits.len() as u32 - i as u32 - 1;
        res += digit * 10i32.pow(exponent);
    }

    res
}

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

pub fn format_list(list: Option<Box<ListNode>>) -> String {
    let mut current = &list;
    let mut elements = Vec::new();

    while let Some(node) = current {
        elements.push(node.val.to_string());
        current = &node.next;
    }

    format!("[{}]", elements.join(", "))
}
