use std::collections::HashMap;

impl Solution {
    pub fn update(map: &mut HashMap<i32, (i32, i32)>, tuple: (i32, i32)) {
        map.insert(tuple.0, tuple);
        map.insert(tuple.1, tuple);
    }

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        // store array element and its minimum and maximum available
        // e.g array is [-4, -3, -2, -1, 0, 5, 6, 7, 8, 9, 10, 11]
        // store in hashmap -4 -> (-4,0)
        let mut map: HashMap<i32, (i32, i32)> = HashMap::new();

        for num in nums {
            match map.get(&num) {
                Some(_) => {}
                None => match (map.get(&(num - 1)), map.get(&(num + 1))) {
                    (Some(min), Some(max)) => {
                        let val = (min.0, max.1);
                        map.insert(num, val);
                        Solution::update(&mut map, val);
                    }
                    (Some(min), None) => {
                        let val = (min.0, num);
                        map.insert(num, val);
                        Solution::update(&mut map, val);
                    }
                    (None, Some(max)) => {
                        let val = (num, max.1);
                        map.insert(num, val);
                        Solution::update(&mut map, val);
                    }
                    (None, None) => {
                        let val = (num, num);
                        map.insert(num, val);
                    }
                },
            }
        }

        let mut max: i32 = 0;
        for (_, val) in map {
            let diff = val.1 - val.0;
            if diff > max {
                max = diff;
            }
        }

        max + 1
    }
}
