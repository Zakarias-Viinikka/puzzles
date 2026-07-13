use std::collections::HashMap;
use std::collections::hash_map::Entry;

/*pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();
    for item in strs.into_iter() {
        let mut chars = item.clone().chars().collect::<Vec<char>>();
        chars.sort();
        let hask_key: String = chars.iter().collect();
        hash_map.entry(hask_key).or_insert(Vec::new()).push(item);
    }

    let x: Vec<Vec<String>> = hash_map.into_iter().map(|(_, value)| value).collect();
    x
}*/

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hash_map: HashMap<String, usize> = HashMap::new();
    let mut ctr: usize = 0;
    let mut output: Vec<Vec<String>> = Vec::new();
    for item in strs.into_iter() {
        let mut chars = item.clone().chars().collect::<Vec<char>>();
        chars.sort();
        let hash_key: String = chars.iter().collect();
        match hash_map.entry(hash_key) {
            Entry::Occupied(e) => {
                output[e.get().clone()].push(item);
            }
            Entry::Vacant(e) => {
                e.insert(ctr);
                output.push(vec![item]);
                ctr += 1;
            }
        }
    }
    output
}

fn main() {
    let strs: Vec<String> = vec![
        "act".to_string(),
        "pots".to_string(),
        "tops".to_string(),
        "cat".to_string(),
        "stop".to_string(),
        "hat".to_string(),
    ];
    print!("{:?}", group_anagrams(strs));
}
/*
Given an array of strings strs, group all anagrams together into sublists. You may return the output in any order.

An anagram is a string that contains the exact same characters as another string, but the order of the characters can be different.

Example 1:

Input: strs = ["act","pots","tops","cat","stop","hat"]

Output: [["hat"],["act", "cat"],["stop", "pots", "tops"]]
 */
