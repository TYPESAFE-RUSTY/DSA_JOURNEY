use std::cmp::Ordering;
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left: i32 = 1;
        let mut right: i32 = *piles.iter().max().unwrap();

        while left < right {
            let mid: i32 = (left + right) / 2;
            let mut hours: i32 = 0;

            piles.iter().for_each(|elem| {
                hours += (*elem as f64 / mid as f64).ceil() as i32;
            });

            match hours.cmp(&h) {
                Ordering::Greater => {
                    left = mid + 1;
                }
                _ => {
                    right = mid;
                }
            }
        }
        return right;
    }
}
