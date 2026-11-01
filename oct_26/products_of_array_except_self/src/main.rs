/*
Given an integer array nums, return an array output where output[i] is the product of all the elements of nums except nums[i].

Each product is guaranteed to fit in a 32-bit integer.

Follow-up: Could you solve it in
O
(
n
)
O(n) time without using the division operation?

Example 1:

Input: nums = [1,2,4,6]

Output: [48,24,12,8]
Example 2:

Input: nums = [-1,0,1,2,3]

Output: [0,-6,0,0,0]
Constraints:

2 <= nums.length <= 100,000
-30 <= nums[i] <= 30
The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
*/

//i looked up the solution because i had no clue how to do it.
// so this is just me trying to implement it
fn main() {
    println!("Hello, world!");
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let (prefix_precalc, suffix_precalc) = create_prefix_and_suffix_precalc(nums);

    finish(prefix_precalc, suffix_precalc)
}

fn create_prefix_and_suffix_precalc(nums: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let left_side = init_left_side(&nums);
    let right_side = init_right_side(&nums);

    (left_side, right_side)
}

fn finish(left_vec: Vec<i32>, right_vec: Vec<i32>) -> Vec<i32> {
    let mut x = Vec::new();
    println!("precalcs from the left: {:?}", left_vec);
    println!("precalcs from the right: {:?}", right_vec);
    for i in 0..left_vec.len() {
        println!("i: {i}");
        x.push(left_vec[i] + right_vec[i])
    }
    x
}

fn init_left_side(n: &Vec<i32>) -> Vec<i32> {
    let mut left_side: Vec<i32> = vec![0; n.len()];
    for i in 0..n.len() {
        /*println!("i: {i}");
        println!("n[i]: {}", n[i]);*/
        let new_prod;
        if i == 0 {
            //println!("inserting 0 at position 0 in left_side");
            left_side[i] = 0;
            //println!("leftside insert: 0");
        } else if i == 1 {
            new_prod = 1 * n[i - 1];
            left_side[1] = new_prod;
            //println!("leftside insert: {}", new_prod);
        } else {
            //println!("inserting at: {i}");
            //println!("{} * {}", { left_side[i - 1] }, { n[i - 1] });
            new_prod = left_side[i - 1] * n[i - 1];
            left_side[i] = new_prod;
            //println!("leftside insert: {}", new_prod);
        }
    }
    //println!("---");

    //println!("final left_side: {:?}", left_side.clone());

    left_side
}

fn init_right_side(n: &Vec<i32>) -> Vec<i32> {
    let mut right_side: Vec<i32> = vec![0; n.len()];
    let l = n.len();
    for i in (0..n.len()).rev() {
        if i == l - 1 {
            right_side[l - 1] = 0;
        } else if i == l - 2 {
            right_side[l - 2] = n[l - 1];
        } else {
            //when i == 2
            //            here
            //right_side [____,number we care,___]
            //nums       [____,number we care,___]
            right_side[i] = right_side[i + 1] * n[i + 1];
            println!("{} * {}", right_side[i + 1], n[i + 1]);
        }
    }
    right_side
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn x() {
        let list = vec![1, 2, 3, 10];
        println!("list being tested: {:?}", list);
        let result = product_except_self(list);
        let expected_result = vec![60, 30, 20, 6];
        assert_eq!(result, expected_result)
    }*/

    /*#[test]
    fn x2() {
        let list = vec![1, 0, 3, 10];
        let result = product_except_self(list);
        let expected_result = vec![0, 30, 0, 0];
        assert_eq!(result, expected_result)
    }*/

    #[test]
    fn test_left_side() {
        let nums = vec![100, 200, 2, 3, 4];

        let result = init_left_side(&nums);
        let expected_result = vec![0, 100, 20000, 40000, 120000];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_right_side() {
        let nums = vec![100, 200, 2, 3, 4];

        let result = init_right_side(&nums);
        let expected_result = vec![4800, 24, 12, 4, 0];

        println!("expected: {:?}", expected_result.clone());
        println!("result: {:?}", result.clone());
        assert_eq!(result, expected_result);
    }

    /*#[test]
    fn iter_logic() {
        let list = vec![5, 10, 2, 3];

        let mut test1 = list.iter().skip(2);
        println!("{:?}", list);

        println!("skip 2 then .next()");
        println!("{:?}", test1.next());

        let mut test2 = list.iter().skip(2).rev();

        println!("skip 2 then rev, then .next()");
        println!("{:?}", test2.next());
        assert_eq!(1, 2)
    }

    #[test]
    fn proper_iter_logic() {
        let list = vec![5, 10, 2, 3, 99];

        let mut test1 = list.iter().skip(2);
        println!("{:?}", list);

        println!("skip 2 then .next()");
        println!("{:?}", test1.next());

        let mut test2 = list.iter().rev().skip(list.len() - 2);

        println!("skip 2 then rev, then .next()");
        println!("{:?}", test2.next());
        assert_eq!(1, 2)
    }*/
}
