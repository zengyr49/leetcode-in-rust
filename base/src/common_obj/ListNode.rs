use std::ptr::null;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val:i32) -> Self {
        ListNode {
            val,
            next:None
        }
    }

    ///
    /// construct node chain for ListNode according to input order
    /// # Example:
    /// ```
    /// use base::common_obj::ListNode::ListNode;
    /// let mut m = vec![1,2,3];
    /// let res = ListNode::construct(m);
    /// assert!(res.unwrap().val == 1);
    /// ```
    /// and it will return a list node starts with value 1 and next node's value is 2 and finally 3
    ///
    pub fn construct(mut args:Vec<i32>) -> Option<Box<ListNode>> {
        if args.is_empty() {
            assert!(false, "please do not input empty value");
        }
        let mut dummy_head = ListNode {
            val: 0,
            next: None
        };
        // since cur node is just a pointer of dummy head, we don't need to hand off ownership
        let mut cur_node = &mut dummy_head;
        for i in args {
            let temp_node = ListNode {
                val: i,
                next: None
            };
            cur_node.next = Some(Box::new(temp_node));
            // don't forget to refresh cur_node
            cur_node = cur_node.next.as_mut().unwrap();
        }
        dummy_head.next
    }
}