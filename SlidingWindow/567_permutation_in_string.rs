impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut reference: [usize; 26] = [0; 26];
        let mut window_hash: [usize; 26] = [0; 26];

        // create a reference
        for (elem1, elem2) in s1.chars().zip(s2.chars()) {
            reference[elem1 as usize - 'a' as usize] += 1;
            window_hash[elem2 as usize - 'a' as usize] += 1;
        }

        let string: Vec<char> = s2.chars().collect();
        let mut temp = string[0];

        // compare reference with window hash and update for every window
        for elem in string.windows(s1.len()).skip(1) {
            if reference == window_hash {
                return true;
            }
            window_hash[*elem.last().unwrap() as usize - 'a' as usize] += 1;
            window_hash[temp as usize - 'a' as usize] -= 1;
            temp = elem[0];
        }

        reference == window_hash
    }
}
