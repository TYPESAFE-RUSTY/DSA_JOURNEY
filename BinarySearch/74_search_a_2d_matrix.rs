impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut rlow = 0;
        let mut rhigh = matrix.len();

        while rlow < rhigh {
            let mid = (rlow + rhigh) / 2;
            if matrix[mid][0] <= target && matrix[mid].last().unwrap() >= &target {
                match matrix[mid].binary_search(&target) {
                    Ok(_) => {
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            } else if matrix[mid][0] > target {
                rhigh = mid;
            } else {
                rlow = mid + 1;
            }
        }

        false
    }
}
