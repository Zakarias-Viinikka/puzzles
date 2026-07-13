use std::collections::{HashMap, HashSet};

fn main() {
    let nums = vec![1, 2, 2, 3, 3, 3];
    let k = 2;
    println!("{:?}", top_k_frequent(nums, k));
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let capacity = nums.len();
    let mut nums_hash = HashMap::with_capacity(capacity);
    let mut nums_sorted: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(capacity);

    for num in nums.iter() {
        let new_appearances = nums_hash.entry(num).or_insert(0);
        *new_appearances += 1;
        update_nums_sorted(&mut nums_sorted, *new_appearances, *num);
    }
    println!("{:#?}", nums_hash);
    println!("{:#?}", nums_sorted);
    let nums_sorted_values: Vec<_> = nums_sorted.into_values().collect();
    let mut result = Vec::new();

    let mut ctr = 0;
    nums_sorted_values.into_iter().rev().for_each(|set| {
        set.into_iter().for_each(|num| {
            if ctr < k {
                result.push(num);
                ctr += 1;
            }
        });
    });
    result
}

fn update_nums_sorted(
    nums_sorted: &mut HashMap<i32, HashSet<i32>>,
    new_appearances: i32,
    num: i32,
) {
    //nums_hash is
    // key =  num
    // val = appearances
    //
    // nums_sorted is
    // key = appearances
    // val = index in nums_hash

    //remove old val
    if let Some(inside) = nums_sorted.get_mut(&(new_appearances - 1)) {
        inside.remove(&num);
    };
    nums_sorted
        .entry(new_appearances)
        .and_modify(|hash_set| {
            hash_set.insert(num);
        })
        .or_insert_with(|| {
            let mut new_hash_set = HashSet::new();
            new_hash_set.insert(num);
            new_hash_set
        });
}

/*
Given an integer array nums and an integer k, return the k most frequent elements within the array.

The test cases are generated such that the answer is always unique.

You may return the output in any order.

Example 1:

Input: nums = [1,2,2,3,3,3], k = 2

Output: [2,3]
 */
