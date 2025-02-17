// Given a list of integers, use a vector and return the median 
// (when sorted, the value in the middle position) and mode 
// (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

pub fn median_and_mode() {
    let mut example1 = vec![3, 1, 4, 1, 5];
    let mut example2 = vec![7, 2, 9, 4, 7];
    let mut example3 = vec![2, 3, 3, 4];
    let mut example4 = vec![1, 2, 3, 4];
    
    println!("{:?}", finding_median_and_mode(&mut example1));
    println!("{:?}", finding_median_and_mode(&mut example2));
    println!("{:?}", finding_median_and_mode(&mut example3));
    println!("{:?}", finding_median_and_mode(&mut example4));

}

fn finding_median_and_mode(v: &mut Vec<i32>) -> (i32, i32) {    
    v.sort();

    let len = v.len();
    let median = if len % 2 == 0 {
        (v[len / 2] + v[(len / 2) - 1]) / 2

    } else {
        v[(len - 1) / 2]
    };

    let mut map = HashMap::new();
    for number in v {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut iterated_map: Vec<(_, _)> = map.into_iter().collect();
    iterated_map.sort_by(|a, b| b.1.cmp(&a.1));

    let mode = *iterated_map[0].0;
    
    return (median, mode);

}