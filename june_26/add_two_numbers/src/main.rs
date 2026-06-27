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

/*
*
*
* You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.


Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [7,0,8]
Explanation: 342 + 465 = 807.
*/

fn main() {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1_reversed: Vec<char> = Vec::new();
        let mut current = l1;
        while current.is_some() {
            list1_reversed
                .push(char::from_digit(current.as_ref().unwrap().val as u32, 10).unwrap());
            current = current.unwrap().next;
        }

        let mut full_num1 = "".to_string();
        list1_reversed
            .iter()
            .rev()
            .for_each(|num| full_num1.push(*num));
        let full_num1: i32 = full_num1.parse().unwrap();

        let mut list2_reversed: Vec<char> = Vec::new();
        let mut current = l2;
        while current.is_some() {
            list2_reversed
                .push(char::from_digit(current.as_ref().unwrap().val as u32, 10).unwrap());
            current = current.unwrap().next;
        }

        let mut full_num2 = "".to_string();
        list2_reversed
            .iter()
            .rev()
            .for_each(|num| full_num2.push(*num));
        let full_num2: i32 = full_num2.parse().unwrap();

        let combined_number = full_num1 + full_num2;

        let mut new_node = ListNode::new(0);
        let first_loop = true;
        let current;
        for digit_char in combined_number.to_string().chars() {
            //let mut inner_node;
            let digit: i32 = digit_char.to_digit(10).unwrap() as i32;
            if first_loop {
                new_node = ListNode::new(digit)
            } else {
                let current = &mut current
                //inner_node = Box::new(ListNode::new(digit));
            }
        }
        Some(Box::new(new_node))
    }
}
