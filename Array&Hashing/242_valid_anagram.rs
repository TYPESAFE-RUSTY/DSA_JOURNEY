use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map: HashMap<char, i32> = HashMap::new();

        for (source_char, target_char) in s.chars().into_iter().zip(t.chars().into_iter()) {
            match map.get(&source_char) {
                Some(count) => {
                    map.insert(source_char, *count + 1);
                }
                None => {
                    map.insert(source_char, 1);
                }
            };
            match map.get(&target_char) {
                Some(count) => {
                    map.insert(target_char, *count - 1);
                }
                None => {
                    map.insert(target_char, -1);
                }
            }
        }

        for item in s.chars().into_iter() {
            match map.get(&item) {
                Some(count) => {
                    if *count != 0 {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_anagram_compressed(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map: HashMap<char, i32> = HashMap::new();

        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);

        map.into_values().all(|val| val == 0)
    }
}
