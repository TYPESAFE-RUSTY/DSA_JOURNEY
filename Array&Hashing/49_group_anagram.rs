use std::collections::HashMap;

impl Solution {
    pub fn generate_key(input: &String) -> [u8; 26] {
        let mut res: [u8; 26] = [0; 26];
        for i in input.chars() {
            res[i as usize - 97] += 1;
        }
        res
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        let mut result: Vec<Vec<String>> = Vec::new();

        for str in strs {
            let key = Solution::generate_key(&str);

            match map.get_mut(&key) {
                Some(item) => {
                    item.push(str.clone());
                }
                None => {
                    let mut vector: Vec<String> = Vec::new();
                    vector.push(str.clone());
                    map.insert(key, vector);
                }
            }
        }

        for (_key, val) in map {
            result.push(val)
        }

        result
    }
}
