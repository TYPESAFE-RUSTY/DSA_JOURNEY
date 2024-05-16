impl Solution {
    fn generate(
        stack: &mut Vec<char>,
        res: &mut Vec<String>,
        n: i32,
        open_count: i32,
        closed_count: i32,
    ) -> () {
        if open_count == closed_count && closed_count == n {
            res.push(stack.iter().collect());
            return;
        }

        if open_count < n {
            stack.push('(');
            Solution::generate(stack, res, n, open_count + 1, closed_count);
            stack.pop();
        }

        if open_count > closed_count {
            stack.push(')');
            Solution::generate(stack, res, n, open_count, closed_count + 1);
            stack.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut stack: Vec<char> = Vec::new();
        let mut res: Vec<String> = Vec::new();
        Solution::generate(&mut stack, &mut res, n, 0, 0);
        res
    }
}
