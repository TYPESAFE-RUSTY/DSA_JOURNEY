impl Solution {
    pub fn clean(s: String) -> String {
        let mut res = String::new();
        for elem in s.chars() {
            if elem.is_alphanumeric() {
                res.push(elem.to_ascii_lowercase());
            }
        }
        res
    }

    pub fn is_palindrome(s: String) -> bool {
        let test_string = Solution::clean(s);

        for ((index, low), high) in test_string
            .chars()
            .enumerate()
            .zip(test_string.chars().rev())
        {
            if index > test_string.len() / 2 {
                return true;
            }
            if low != high {
                return false;
            }
        }
        true
    }

    pub fn is_palindrome_elegant(s: String) -> bool {
        let iter = s
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .map(|character| character.to_ascii_lowercase());

        iter.clone().eq(iter.rev())
    }
}
