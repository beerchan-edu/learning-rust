// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/559/

pub fn solution_plus_one() {
let digits1: Vec<i8> = vec![1,2,3];
let digits2: Vec<i8> = vec![4,3,2,1];
let digits3: Vec<i8> = vec![9];

println!("{:?}", plus_one(digits1));
println!("{:?}", plus_one(digits2));
println!("{:?}", plus_one(digits3));

}

fn plus_one(mut digits: Vec<i8>) -> Vec<i8> {
    for digit in digits.iter_mut().rev() {
        if *digit < 9 {
            *digit += 1;
            return digits;
        } else {
        *digit = 0;
        }
    }
    digits.insert(0, 1);
    return  digits;
}