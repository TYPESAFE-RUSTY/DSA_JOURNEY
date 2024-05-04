use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut map: HashMap<i32, u16> = HashMap::new();
        let mut count: Vec<Vec<i32>> = Vec::from([Vec::new()]);
        nums.iter().for_each(|elem| {
            *map.entry(*elem).or_insert(0) += 1;
            count.push(Vec::new());
        });
        for (key, val) in map {
            let arr = count.get_mut(val as usize).unwrap();
            arr.push(key);
        }
        for items in count.iter().rev() {
            for item in items {
                res.push(*item);
                if res.len() == k as usize {
                    return res;
                }
            }
        }
        res
    }
}
