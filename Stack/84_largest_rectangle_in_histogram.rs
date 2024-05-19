impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut stack: Vec<(usize, i32)> = Vec::new();

        for (index, elem) in heights.iter().enumerate() {
            let mut current = index;
            while !stack.is_empty() && stack.last().unwrap().1 > *elem {
                let popped = stack.pop().unwrap();
                max = i32::max(popped.1 * (index - popped.0) as i32, max);
                current = popped.0;
            }
            stack.push((current, *elem));
        }
        stack
            .iter()
            .for_each(|(index, elem)| max = i32::max(max, *elem * (heights.len() - *index) as i32));
        max
    }
}
