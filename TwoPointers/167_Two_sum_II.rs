impl Solution {
    pub fn binary_search(array: &Vec<i32>, start: usize, target: i32) -> Option<usize> {
        let mut low = start;
        let mut high = array.len() - 1;
        let mut mid: usize;

        while low <= high {
            mid = (low + high) / 2;
            if array[mid] == target {
                return Some(mid);
            } else if array[mid] > target {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        None
    }

    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];

        for (index, number) in numbers.iter().enumerate() {
            let target = target - number;
            match Solution::binary_search(&numbers, index + 1, target) {
                Some(i) => {
                    res.push(index as i32 + 1);
                    res.push(i as i32 + 1);
                    return res;
                }
                None => {}
            }
        }
        unreachable!("constraint unsatisfied")
    }

    pub fn two_sum_elegant(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for (index, number) in numbers.iter().enumerate() {
            if let Ok(j) = numbers[index + 1..].binary_search(&(&target - number)) {
                return vec![(index + 1) as i32, (index + 1 + j + 1) as i32];
            }
        }
        unreachable!("constraint unsatisfied")
    }
}
