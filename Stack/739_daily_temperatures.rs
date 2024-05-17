impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        temperatures.iter().for_each(|_| res.push(0));
        let mut stack: Vec<(usize, &i32)> = Vec::new();

        for (index, elem) in temperatures.iter().enumerate() {
            while !stack.is_empty() && stack.last().unwrap().1 < elem {
                let temp = stack.pop().unwrap();
                res[temp.0] = (index - temp.0) as i32;
            }
            stack.push((index, elem))
        }

        res
    }
}
