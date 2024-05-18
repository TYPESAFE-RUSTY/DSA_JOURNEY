impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut time: Vec<(i32, f32)> = Vec::new();
        for (distance, velocity) in position.iter().zip(&speed) {
            let actual_distance = (target - distance) as f32;
            let calculated_time = actual_distance / *velocity as f32;
            time.push((*distance, calculated_time));
        }
        time.sort_by(|a, b| a.0.cmp(&b.0));

        let mut stack: Vec<f32> = Vec::new();
        for elem in time.iter().rev() {
            if !stack.is_empty() && stack.last().unwrap() >= &elem.1 {
                continue;
            }
            stack.push(elem.1)
        }

        stack.len() as i32
    }
}
