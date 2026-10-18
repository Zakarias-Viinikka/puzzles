/*thoughts after finishing.
 *
 * my 'smart' rc idea where i avoid cloning is apparently more expensive since cloning a char is cheap and making a new rc for every char is not
 *
 * the other thing is that the optimal solution turns chars into numbers so you use an array instead of a hashmap for looking up the letters ['s' as usize]
 */

use std::{
    collections::{
        HashMap,
        hash_map::Entry::{Occupied, Vacant},
    },
    rc::Rc,
};
//Given a string s, find the first non-repeating
// character in it and return its index.
// If it does not exist, return -1.
pub fn first_uniq_char(s: String) -> i32 {
    let x = make_hashmap_and_sorted_list(s);
    let map = x.map;
    let sorted_list = x.sorted_list;

    for character in sorted_list.into_iter() {
        let c = *character;
        match map.get(&c) {
            Some((count, index)) => {
                if *count == 1 {
                    return *index as i32;
                }
            }
            None => {}
        }
    }
    -1
}

//i need to read the entire thing
// hashmaps would let me read it once and know if there is a duplicate
// this requires 2 steps i think
// make the hashmap. then go through every letter, but you want to go through every letter only once, and you
// want to quit the moment you find a unique one.
//
// so i need a vec or something, so i can have a sorted lsit of the letters. a tuple? no. the value is the key, i just want it sorted

fn make_hashmap_and_sorted_list(s: String) -> MapAndSortedList {
    let mut map: HashMap<Rc<char>, (u64, u64)> = HashMap::new();
    let mut sorted_list: Vec<Rc<char>> = Vec::with_capacity(100000);

    for (index, character) in s.chars().enumerate() {
        let shared_character = Rc::new(character);
        let is_new_entry =
            add_to_map_and_say_if_new_entry(&mut map, shared_character.clone(), index as u64);
        if is_new_entry {
            sorted_list.push(shared_character);
        }
    }

    MapAndSortedList { map, sorted_list }
}

fn add_to_map_and_say_if_new_entry(
    map: &mut HashMap<Rc<char>, (u64, u64)>,
    char: Rc<char>,
    index: u64,
) -> bool {
    match map.entry(char) {
        Occupied(mut x) => {
            let value = x.get_mut();
            value.0 += 1;
            return false;
        }
        Vacant(x) => {
            x.insert((1, index));
            return true;
        }
    }
}

struct MapAndSortedList {
    map: HashMap<Rc<char>, (u64, u64)>,
    sorted_list: Vec<Rc<char>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_adding_to_map_works_correctly() {
        let mut test_map: HashMap<Rc<char>, (u64, u64)> = HashMap::new();
        let character_that_already_exists_1: Rc<char> = Rc::new('o');
        let character_that_already_exists_2: Rc<char> = Rc::new('t');
        let character_that_already_exists_3: Rc<char> = Rc::new('e');
        let character_that_already_exists_4: Rc<char> = Rc::new('f');

        test_map.insert(character_that_already_exists_1, (1, 0));
        test_map.insert(character_that_already_exists_2, (1, 1));
        test_map.insert(character_that_already_exists_3, (1, 2));
        test_map.insert(character_that_already_exists_4, (1, 3));

        let result1 = add_to_map_and_say_if_new_entry(&mut test_map, Rc::new('e'), 4);
        let expected_result1 = false;

        let result2 = add_to_map_and_say_if_new_entry(&mut test_map, Rc::new('e'), 5);
        let expected_result2 = false;

        let result3 = add_to_map_and_say_if_new_entry(&mut test_map, Rc::new('x'), 6);
        let expected_result3 = true;

        assert_eq!(result1, expected_result1);
        assert_eq!(result2, expected_result2);
        assert_eq!(result3, expected_result3);
    }

    #[test]
    fn does_map_and_sorted_list_match_expectations() {
        let (map, sorted_list) = {
            let x = make_hashmap_and_sorted_list("hello".to_string());
            (x.map, x.sorted_list)
        };

        assert_eq!(map.len(), 4);
        assert_eq!(map.get(&rc_letter("h")), Some(&(1, 0)));
        assert_eq!(map.get(&rc_letter("e")), Some(&(1, 1)));
        assert_eq!(map.get(&rc_letter("l")), Some(&(2, 2)));
        assert_eq!(map.get(&rc_letter("o")), Some(&(1, 4)));

        assert_eq!(sorted_list.len(), 4);
        assert_eq!(sorted_list[0], rc_letter("h"));
        assert_eq!(sorted_list[1], rc_letter("e"));
        assert_eq!(sorted_list[2], rc_letter("l"));
        assert_eq!(sorted_list[3], rc_letter("o"));
    }

    fn rc_letter(letter: &str) -> Rc<char> {
        match letter.chars().next() {
            Some(c) => Rc::new(c),
            None => panic!("rc_letter got an empty string"),
        }
    }
}
