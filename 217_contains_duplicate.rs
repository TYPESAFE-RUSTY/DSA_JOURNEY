use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        let mut index: i32 = 0;
        for num in nums {
            let option = hash.get(&num);
            match option {
                Some(a) => {
                    return true;
                }
                None => hash.insert(num, index),
            };
            index = index + 1;
        }
        false
    }
}
