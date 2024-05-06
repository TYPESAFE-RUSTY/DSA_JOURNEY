impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut colcheck: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut boxcheck: Vec<Vec<bool>> = vec![vec![false; 9]; 3];
        for (i, row) in board.iter().enumerate() {
            // reinitalize to false
            if i % 3 == 0 && i != 0 {
                boxcheck = vec![vec![false; 9]; 3];
            }
            let mut check: Vec<bool> = vec![false; 9];
            for (j, content) in row.iter().enumerate() {
                // check for .
                if *content == '.' {
                    continue;
                }
                // convert to get usize to index
                let data = content.to_digit(10).unwrap() as usize - 1;
                let box_index = (j - j % 3) / 3;

                println!("{}{}", box_index, data + 1);
                // check for box
                if !boxcheck[box_index][data] {
                    boxcheck[box_index][data] = true;
                    for array in &boxcheck {
                        for content in array {
                            print!("{}\t", content);
                        }
                        println!();
                    }
                } else {
                    return false;
                }

                // check for col
                if !colcheck[j][data] {
                    colcheck[j][data] = true;
                } else {
                    return false;
                }

                // check for row
                if !check[data] {
                    check[data] = true;
                } else {
                    return false;
                }
            }
        }

        true
    }
}
