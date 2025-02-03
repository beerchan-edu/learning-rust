pub fn reverse_string(s: &mut Vec<char>) {
    let mut i: usize = 0;
    let mut j = s.len() - 1;
    while j > i {
        let tmp = s[i];
        s[i] = s[j];
        s[j] = tmp;
        i += 1;
        j -= 1;
    }
}

pub fn reverse(x: i32) -> i32 {
    // 123 -> 3 -> 32 -> 321
    if x == i32::MIN {
        return 0;
    }
    let mut result: i32 = 0;
    let mut reminder: i32 = x.abs();
    while reminder > 0 {
        let digit = reminder % 10;
        reminder = reminder / 10;
        match result.checked_mul(10) {
            None => return 0,
            Some(new_result) => match new_result.checked_add(digit) {
                None => return 0,
                Some(new_result) => {
                    result = new_result;
                }
            },
        }
    }
    if x < 0 {
        match result.checked_mul(-1) {
            None => return 0,
            Some(result) => return result,
        }
    }
    result
}
