use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    let mut ctr = 0;
    for num in nums.iter() {
        let matching_num = target - num.clone();
        if hash_map.get(&matching_num).is_some() {
            return vec![*hash_map.get(&matching_num).unwrap(), ctr];
        } else {
            hash_map.insert(*num, ctr);
        }
        ctr += 1;
    }
    vec![]
}

/*
Input:
nums = [3,4,5,6], target = 7

Output: [0,1]
 */
fn main() {
    let nums = vec![-2, 1, 0, 2];
    let target = 1;
    println!("{:?}", two_sum(nums, target));
}
