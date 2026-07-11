use std::collections::HashMap;

/*impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut hash_storage = HashMap::new();
        for num in nums.into_iter() {
            println!(num);
            if hash_storage.len() != 0 {
                println!("Option Status: {:#?}", hash_storage.get(&num));
                if hash_storage.get(&num).is_some() {
                    return true;
                }
            } else {
                hash_storage.insert(num, "");
            }
        }
        false
    }
}*/

fn main() {
    //let nums = [1, 2, 3, 3];
    let mut hash_storage = HashSet::new();
    if nums.len() > 1 {
        hash_storage.insert(nums[0]);
    }
    for num in nums.into_iter().skip(1) {
        if hash_storage.get(&num).is_some() {
            return true;
        }
        hash_storage.insert(num);
    }
    false
}
