/// Given two non-empty linked lists representing two non-negative integers stored in reverse
/// order. Each of their nodes contians a single digit. Add the two numbers return the sum as a
/// linked list.
pub struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            carry = sum / 10;
            let digit = sum % 10;
            let new_node = Box::new(ListNode::new(digit));
            *tail = Some(new_node);
            tail = &mut tail.as_mut().unwrap().next;
        }
        head
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
    pub fn format_list(list: Option<Box<ListNode>>) -> String {
        let mut current = &list;
        let mut elements = Vec::new();

        while let Some(node) = current {
            elements.push(node.val.to_string());
            current = &node.next;
        }

        format!("[{}]", elements.join(", "))
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct LinkedListNumber {
    head: Option<Box<ListNode>>,
}

impl LinkedListNumber {
    fn new() -> Self {
        LinkedListNumber { head: None }
    }

    fn push(&mut self, val: i32) {
        let new_node = Box::new(ListNode {
            val: val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    fn from_array(arr: &[u8]) -> Self {
        let mut new_linked_list = LinkedListNumber::new();
        for item in arr {
            new_linked_list.push((*item).into());
        }
        new_linked_list.reversed()
    }

    fn reversed(&mut self) -> Self {
        let mut prev = None;
        let mut curr = self.head.clone();

        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        LinkedListNumber { head: prev }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_nums_success() {
        // [2, 4, 3]
        let list1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: (3),
                    next: None,
                })),
            })),
        }));
        // [5, 6, 4]
        let list2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: (4),
                    next: None,
                })),
            })),
        }));
        // [7, 0, 8];
        let output = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: (8),
                    next: None,
                })),
            })),
        }));

        let added_nums = Solution::add_two_numbers(list1, list2);
        // [2, 4, 3] + [5, 6, 4] -> [7, 0, 8]
        assert_eq!(added_nums, output);
    }

    #[test]
    fn add_nums_success3() {
        // [5, 6];
        let list1 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode { val: 6, next: None })),
        }));
        // [5, 4, 9];
        let list2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: (9),
                    next: None,
                })),
            })),
        }));
        // let output = [0, 1, 0, 1];
        let output = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: (0),
                    next: Some(Box::new(ListNode {
                        val: (1),
                        next: None,
                    })),
                })),
            })),
        }));

        let added_nums = Solution::add_two_numbers(list1, list2);
        // [5, 6] + [5, 4, 9] -> [0, 1, 0, 1]
        assert_eq!(added_nums, output);
    }

    #[test]
    fn add_nums_success2() {
        // [9, 9, 9, 9, 9, 9, 9];
        let list1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: (9),
                    next: Some(Box::new(ListNode {
                        val: (9),
                        next: Some(Box::new(ListNode {
                            val: (9),
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: (9),
                                    next: None,
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        // [9, 9, 9, 9];
        let list2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: (9),
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        }));
        // let output = [8, 9, 9, 9, 0, 0, 0, 1];
        let output = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: (9),
                    next: Some(Box::new(ListNode {
                        val: (9),
                        next: Some(Box::new(ListNode {
                            val: (0),
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: (0),
                                    next: Some(Box::new(ListNode { val: 1, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        let added_nums = Solution::add_two_numbers(list1, list2);
        // [9,9,9,9,9,9,9] + [9,9,9,9] -> [8,9,9,9,0,0,0,1]
        assert_eq!(added_nums, output);
    }

    #[test]
    fn linked_list_reverse_success() {
        // [2, 4, 3]
        let list1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: (3),
                    next: None,
                })),
            })),
        }));
        // [5, 6, 4]
        let list2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: (4),
                    next: None,
                })),
            })),
        }));
        // [3, 4, 2]
        let output1 = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: (2),
                    next: None,
                })),
            })),
        }));
        // [4, 6, 5]
        let output2 = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: (5),
                    next: None,
                })),
            })),
        }));

        // [2, 4, 3] -> [3, 4, 2]
        let reversed_list1 = Solution::reverse_linked_list(list1);
        // [5, 6, 4] -> [4, 6, 5]
        let reversed_list2 = Solution::reverse_linked_list(list2);
        assert_eq!(reversed_list1, output1);
        assert_eq!(reversed_list2, output2);
    }

    #[test]
    fn linked_list_number_reverse_success() {
        // [2, 4, 3]
        let mut list1 = LinkedListNumber {
            head: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: (3),
                        next: None,
                    })),
                })),
            })),
        };
        // [5, 6, 4]
        let mut list2 = LinkedListNumber {
            head: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: (4),
                        next: None,
                    })),
                })),
            })),
        };
        // [3, 4, 2]
        let output1 = LinkedListNumber {
            head: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: (2),
                        next: None,
                    })),
                })),
            })),
        };
        // [4, 6, 5]
        let output2 = LinkedListNumber {
            head: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: (5),
                        next: None,
                    })),
                })),
            })),
        };

        // [2, 4, 3] -> [3, 4, 2]
        let reversed_list1 = list1.reversed();
        // [5, 6, 4] -> [4, 6, 5]
        let reversed_list2 = list2.reversed();
        assert_eq!(reversed_list1, output1);
        assert_eq!(reversed_list2, output2);
    }

    #[test]
    fn linked_list_number_from_array_success() {
        let array1: &[u8] = &[2, 4, 3];
        let array2: &[u8] = &[5, 6, 4];
        let output1 = LinkedListNumber {
            head: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: (3),
                        next: None,
                    })),
                })),
            })),
        };
        let output2 = LinkedListNumber {
            head: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: (4),
                        next: None,
                    })),
                })),
            })),
        };

        let linked_list1 = LinkedListNumber::from_array(array1);
        let linked_list2 = LinkedListNumber::from_array(array2);
        assert_eq!(linked_list1, output1);
        assert_eq!(linked_list2, output2);
    }

    //#[test]
    //fn success() {
    //    let linked_list1 = [2, 4, 3];
    //    let linked_list2 = [5, 6, 4];
    //    let output = [7, 0, 8];
    //}
}
