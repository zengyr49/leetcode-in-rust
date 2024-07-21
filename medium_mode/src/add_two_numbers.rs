use base::common_obj::ListNode::ListNode;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
    carried(l1, l2, 0)
}

///
/// this is for iteration
/// carry means the sum of current node value
pub fn carried( l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
    // calculate carry
    if l1.is_none() && l2.is_none() && carry == 0 {
        // border
        None
    } else {
        // calculate carry
        if l1 != None {
            carry += l1.clone().unwrap().val;
        }
        if l2 != None {
            carry += l2.clone().unwrap().val;
        }

        // add to get carried
        Some(Box::new(ListNode {
            next: carried(l1.and_then(|x| {x.next}),
                          l2.and_then(|x| {x.next}),
                          carry/10),
            val: carry % 10
        }))
    }
}

#[cfg(test)]
mod tests {
    use base::common_obj::ListNode::ListNode;

    #[test]
    pub fn test_add_two_numbers() {
        // construct 123 and 456
        let cur_list_node = ListNode::construct(vec![1, 2, 3]);
        println!("{}", cur_list_node.unwrap().val);
    }
}