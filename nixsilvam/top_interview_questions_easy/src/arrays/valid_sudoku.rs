// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/769/

use std::collections::HashSet;

pub fn valid_sudoku () {
let board1: Vec<Vec<char>> = vec![
    vec!['5','3','.','.','7','.','.','.','.'],
    vec!['6','.','.','1','9','5','.','.','.'],
    vec!['.','9','8','.','.','.','.','6','.'],
    vec!['8','.','.','.','6','.','.','.','3'],
    vec!['4','.','.','8','.','3','.','.','1'],
    vec!['7','.','.','.','2','.','.','.','6'],
    vec!['.','6','.','.','.','.','2','8','.'],
    vec!['.','.','.','4','1','9','.','.','5'],
    vec!['.','.','.','.','8','.','.','7','9'],
];

let board2: Vec<Vec<char>> = vec![
    vec!['8','3','.','.','7','.','.','.','.'],
    vec!['6','.','.','1','9','5','.','.','.'],
    vec!['.','9','8','.','.','.','.','6','.'],
    vec!['8','.','.','.','6','.','.','.','3'],
    vec!['4','.','.','8','.','3','.','.','1'],
    vec!['7','.','.','.','2','.','.','.','6'],
    vec!['.','6','.','.','.','.','2','8','.'],
    vec!['.','.','.','4','1','9','.','.','5'],
    vec!['.','.','.','.','8','.','.','7','9'],
];

println!("{}", is_valid_sudoku(board1));
println!("{}", is_valid_sudoku(board2));
}

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // check the rows
    for row in &board {
        let mut seen = HashSet::new();
        for &item in row {           
            if item != '.' {
                if !seen.insert(item) {
                    return false
                }
            }
        }
    }

    // check the columns
    for col_idx in 0..9 {
        let mut seen = HashSet::new();
        for row in &board {
            let val = row[col_idx];
            if val != '.' {
                if !seen.insert(val) {
                    return false
                }
            }
        }
    }

    // check the squares 3x3
    for box_row in (0..9).step_by(3) {
        for box_col in (0..9).step_by(3) {
            let mut seen = HashSet::new();
            for row in 0..3 {
                for col in 0..3 {
                    let r = box_row + row;
                    let c = box_col + col;
                    let val = board[r][c];
                    if val != '.' {
                        if !seen.insert(val) {
                            return false
                        }
                    }
                }
            }
        }
    } 

    return true
}