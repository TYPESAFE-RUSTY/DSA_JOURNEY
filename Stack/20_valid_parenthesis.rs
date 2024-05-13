impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for elem in s.chars() {
            match elem {
                '(' | '{' | '[' => {
                    stack.push(elem);
                }
                ')' | '}' | ']' => {
                    if let Some(top) = stack.pop() {
                        if (elem == ')' && top != '(')
                            || (elem == ']' && top != '[')
                            || (elem == '}' && top != '{')
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }

        stack.is_empty()
    }
}
