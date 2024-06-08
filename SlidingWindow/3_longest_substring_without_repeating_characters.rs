use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring_fast(s: String) -> i32 {
        // saving location where copy was found helps avoid extra loop to remove characters from set
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut start: usize = 0;
        let mut res: usize = 0;

        for (index, elem) in s.chars().enumerate() {
            if let Some(copy) = map.insert(elem, index) {
                res = usize::max(res, index - start);
                start = usize::max(copy + 1, start);
            }
        }

        // substring in the end
        res = usize::max(res, s.len() - start);

        res as i32
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashSet<char> = HashSet::new();
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut res: usize = 0;

        while j < s.len() {
            let hash = s.chars().nth(j).unwrap();
            while map.contains(&hash) {
                map.remove(&s.chars().nth(i).unwrap());
                i += 1;
            }
            map.insert(hash);
            res = usize::max(res, j - i + 1);
            j += 1;
        }

        res as i32
    }
}
