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
    let mut pre_vec: Vec<i32> = Vec::new();
    let mut suf_vec: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        pre_vec.push(get_prod_right(&nums, i as i32));
        suf_vec.push(get_prod_left(&nums, i as i32))
    }
    (pre_vec, suf_vec)
}

fn finish(pre_vec: Vec<i32>, suf_vec: Vec<i32>) -> Vec<i32> {
    let mut x = Vec::new();
    for i in 0..pre_vec.len() {
        x.push(pre_vec[i] * suf_vec[i])
    }
    x
}

fn get_prod_right(nums: &Vec<i32>, i: i32) -> i32 {
    let i = i as usize;
    nums.iter()
        .skip(i + 1)
        .fold(1, |accumulative_result, num| accumulative_result * num)
}

fn get_prod_left(nums: &Vec<i32>, i: i32) -> i32 {
    //println!("{:?}, {}", nums, i);
    let i = i as usize;
    if i > 0 {
        return nums
            .iter()
            .rev()
            .skip(nums.len() - i)
            .fold(1, |accumulative_result, num| {
                //println!("{accumulative_result}, {num}");
                accumulative_result * num
            });
    } else {
        return 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x() {
        let list = vec![1, 2, 3, 10];
        let result = product_except_self(list);
        let expected_result = vec![60, 30, 20, 6];
        assert_eq!(result, expected_result)
    }

    fn x2() {
        let list = vec![1, 0, 3, 10];
        let result = product_except_self(list);
        let expected_result = vec![0, 30, 0, 0];
        assert_eq!(result, expected_result)
    }

    #[test]
    fn prod_right_logic() {
        let nums = vec![100, 200, 2, 3, 4];
        let i = 1;

        let result = get_prod_right(&nums, i);
        let expected_result = 24;
        assert_eq!(result, expected_result);

        let i = 0;

        let result = get_prod_right(&nums, i);
        let expected_result = 4800;
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

    #[test]
    fn prod_left_logic() {
        let nums = vec![100, 200, 2, 3, 4];
        let i = 1;

        let result = get_prod_left(&nums, i);
        let expected_result = 100;
        assert_eq!(result, expected_result);
        println!("left logic passed first test");

        /*
        let i = 2;

        let result = get_prod_left(&nums, i);
        let expected_result = 20000;
        assert_eq!(result, expected_result);
        println!("left logic passed second test");

        let i = 0;

        let result = get_prod_left(&nums, i);
        let expected_result = 0;
        assert_eq!(result, expected_result);
        println!("left logic passed third test");
        */
    }
}
