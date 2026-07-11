use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let hash_map1 = fill_hash_map(s);
        let hash_map2 = fill_hash_map(t);
        if hash_map1 == hash_map2 { true } else { false }
    }
}

fn main() {
    println!("Hello, world!");
}

fn fill_hash_map(letters: String) -> HashMap<char, u8> {
    let mut hash_map = HashMap::new();
    for letter in letters.chars().into_iter() {
        if let Some(hash_value) = hash_map.get_mut(&letter) {
            *hash_value += 1;
        } else {
            hash_map.insert(letter, 1u8);
        }
    }
    hash_map
}
