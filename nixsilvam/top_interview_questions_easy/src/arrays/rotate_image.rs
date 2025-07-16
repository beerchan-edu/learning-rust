// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/770/

pub struct Solution {}

impl Solution {
    pub fn rotate_image_solution () {
        let mut matrix1 = vec![vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9]];

        let mut matrix2 = vec![vec![5,1,9,11],
            vec![2,4,8,10],
            vec![13,3,6,7],
            vec![15,14,12,16]];

        Solution::rotate(&mut matrix1);
        Solution::rotate(&mut matrix2);

        println!("{:?}", matrix1);
        println!("{:?}", matrix2);
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for r in 0..n {
            for c in  r + 1..n {
                let temp = matrix[r][c];
                matrix[r][c] = matrix[c][r];
                matrix[c][r] = temp;
            }
        }
        for i in 0..n {
            matrix[i].reverse();
        }
    }
}

#[allow(dead_code)]
fn main () {
    Solution::rotate_image_solution();
}
 