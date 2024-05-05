impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];

        let mut temp: i32 = 1;
        nums.iter().enumerate().for_each(|(index, elem)| {
            res[index] = temp;
            temp *= elem;
        });

        temp = 1;

        nums.iter().rev().enumerate().for_each(|(index, elem)| {
            let i = nums.len() - 1 - index;
            res[i] *= temp;
            temp *= elem;
        });

        res
    }
}
