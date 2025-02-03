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

pub fn first_uniq_char(s: String) -> i32 {
    let alphabet_len: i32 = 'z' as i32 - 'a' as i32 + 1;
    let mut letters = vec![-1; alphabet_len as usize];
    for (i, elem) in s.char_indices() {
        let position: usize = elem as usize - 'a' as usize;
        if letters[position] == -1 {
            letters[position] = i as i32;
        } else if letters[position] >= 0 {
            letters[position] = -2;
        }
    }
    print!("{:?}", letters);
    if let Some(&min_non_negative_index) = letters.iter().filter(|&&x| x >= 0).min() {
        return min_non_negative_index;
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_uniq_char_test() {
        let result = first_uniq_char(String::from("loveleetcode"));
        assert_eq!(result, 2);
    }
}
