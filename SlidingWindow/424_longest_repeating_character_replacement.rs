impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut map: [usize; 26] = [0; 26];
        let mut max: usize = 0;

        let mut low = 0;
        let mut res = 0;
        for (index, elem) in s.chars().enumerate() {
            map[elem as usize - 65] += 1;
            max = usize::max(max, map[elem as usize - 65]);

            while (index - low + 1) - max > k as usize && low < s.len() {
                map[s.chars().nth(low).unwrap() as usize - 65] -= 1;
                low += 1;
            }
            res = usize::max(res, index - low + 1)
        }
        res as i32
    }
}
