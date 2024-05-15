impl Solution {
    pub fn eval(stack: &mut Vec<i32>, operator: String) {
        let next = stack.pop().unwrap();
        let first = stack.pop().unwrap();

        match operator.as_str() {
            "+" => stack.push(first + next),
            "-" => stack.push(first - next),
            "*" => stack.push(first * next),
            "/" => stack.push(first / next),
            _ => {
                unreachable!()
            }
        }
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for elem in tokens {
            match elem.as_str() {
                "+" | "-" | "*" | "/" => Solution::eval(&mut stack, elem),
                _ => {
                    if let Ok(val) = i32::from_str_radix(&elem, 10) {
                        stack.push(val);
                    } else {
                        println!("vec contained letters!\tConstraints unsatisfied Error");
                    }
                }
            }
        }

        *stack.first().unwrap()
    }
}
