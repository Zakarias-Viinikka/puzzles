/*
https://neetcode.io/problems/top-k-elements-in-list/question?list=neetcode150
Given an integer array nums and an integer k, return the k most frequent elements within the array.

The test cases are generated such that the answer is always unique.

You may return the output in any order.

Example 1:

Input: nums = [1,2,2,3,3,3], k = 2

Output: [2,3]
Example 2:

Input: nums = [7,7], k = 1

Output: [7]
*/

use std::collections::HashMap;

//get a list with numbers, get a k.
// k represents "give me the k most frequent numbers"
//
// first i go over nums and then update a hashmap
// but this is a medium puzzle so i assume the difficult is how you're supposed to read the results?
//
// i could make a separate list and put in numbers at the end when they're new and move them up one when
// they reoccur. and when i move them up i first check "how often does the number above me occur" by looking
// in the hashmap, and if it's less than the occurences of the number being moved, then it moves up
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let (_, sorted_list) = create_hashmap_and_sorted_list(nums.clone());

    let mut answer = vec![];
    for x in 0..k {
        answer.push(sorted_list[x as usize].nr as i32)
    }
    answer
}

type Map = HashMap<u32, u32>;
//the hashmap value should just be the position of a number in the sorted list. and the sorted list carries the amount of times the number appears
pub fn create_hashmap_and_sorted_list(nums: Vec<i32>) -> (Map, Vec<Number>) {
    let mut map: Map = HashMap::new();
    let mut sorted_list: Vec<Number> = Vec::new();
    for n in nums.into_iter() {
        let n = n as u32;
        match map.remove(&n) {
            Some(mut final_position) => {
                //println!("nr that reocurred: {}", n);
                sorted_list[final_position as usize].times_seen += 1;
                loop {
                    let position = final_position as usize;
                    let number: Number = sorted_list[position].clone();
                    match position.checked_sub(1) {
                        Some(position_above) => {
                            let number_above = sorted_list[position_above].clone();
                            if number_above.times_seen < number.times_seen {
                                sorted_list.swap(position, position_above);
                                let above_nr = map.get_mut(&number_above.nr).unwrap();
                                *above_nr += 1;
                                final_position -= 1;
                                //keep going
                            } else {
                                break;
                            }
                        }
                        None => {
                            final_position = 0;
                            break;
                        }
                    }
                }
                map.insert(n, final_position);
            }
            None => {
                sorted_list.push(Number::new(n as u32));
                map.insert(n, sorted_list.len() as u32 - 1);
            }
        }
        //*map.entry(n as u32).or_insert(0) += 1u32;
    }
    (map, sorted_list)
}
#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    nr: u32,
    times_seen: u32,
}

impl Number {
    fn new(nr: u32) -> Self {
        Number {
            nr: nr,
            times_seen: 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sevenseven() {
        let result = top_k_frequent(vec![7, 7], 1);
        assert_eq!(result[0], 7);
    }

    #[test]
    fn mega_tester() {
        let list = vec![7, 7, 6, 7, 4, 4];
        let (map, list) = create_hashmap_and_sorted_list(list);

        let list_expectations = [
            Number {
                nr: 7,
                times_seen: 3,
            },
            Number {
                nr: 4,
                times_seen: 2,
            },
            Number {
                nr: 6,
                times_seen: 1,
            },
        ];
        println!("hashmap: {:?}", map);
        println!("sorted list: {:?}", list);
        assert_eq!(list, list_expectations);

        assert_eq!(*map.get(&7).unwrap() as u32, 0);
        assert_eq!(*map.get(&4).unwrap() as u32, 1);
        assert_eq!(*map.get(&6).unwrap() as u32, 2);
    }

    #[test]
    fn rust_check() {
        let list = vec![5];
        println!(
            "if list has one item, index it by 'len() - 1': {}",
            list[list.len() - 1]
        );
    }
}
