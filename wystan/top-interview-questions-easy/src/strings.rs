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

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut i = 0;
    let mut s1: Vec<char> = s.chars().collect();
    let mut s2: Vec<char> = t.chars().collect();
    s1.sort();
    s2.sort();
    while i < s.len() {
        if s1[i] != s2[i] {
            return false;
        }
        i += 1;
    }
    return true;
}

pub fn is_palindrome(s: String) -> bool {
    let s: Vec<char> = s
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    let mut i = 0;
    let mut j = s.len() - 1;
    if s.len() == 0 {
        return true;
    }

    while j > i {
        if s[i] != s[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

pub fn my_atoi(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let s: Vec<char> = s.chars().collect();
    let mut i: usize = 0;
    // 1. Whitespace: Ignore any leading whitespace
    while i < s.len() {
        if s[i] == ' ' {
            i += 1;
            continue;
        }
        break;
    }
    if i == s.len() {
        return 0;
    }
    // 2 Signedness: Determine the sign by checking if the next character is '-' or '+', assuming positivity if neither present.
    let mut is_positive = true;
    if s[i] == '-' {
        is_positive = false;
        i += 1;
    } else if s[i] == '+' {
        i += 1;
    }
    if i == s.len() {
        return 0;
    }
    // Conversion: Read the integer by skipping leading zeros
    while i < s.len() {
        if s[i] == '0' {
            i += 1;
        }
        break;
    }
    if i == s.len() {
        return 0;
    }

    let mut result: i32 = 0;
    // first digit
    if let Some(digit) = s[i].to_digit(10) {
        if is_positive {
            result = digit as i32;
        } else {
            result = -1 * (digit as i32);
        }
        i += 1;
    } else {
        return 0;
    }

    // remaining digits
    println!("result {}", result);

    while i < s.len() {
        if let Some(digit) = s[i].to_digit(10) {
            if let Some(value) = result.checked_mul(10) {
                println!("{} {}", digit, value);
                let value_to_add = if is_positive {
                    digit as i32
                } else {
                    -1 * (digit as i32)
                };
                if let Some(new_result) = value.checked_add(value_to_add) {
                    println!("{} {} {}", digit, value, new_result);
                    result = new_result;
                    i += 1;
                } else {
                    if result > 0 {
                        return i32::MAX;
                    }
                    return i32::MIN;
                }
            } else {
                if result > 0 {
                    return i32::MAX;
                }
                return i32::MIN;
            }
        } else {
            return result;
        }
    }

    result
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() > haystack.len() {
        return -1;
    }
    if needle.len() == 0 {
        return 0;
    }

    let needle: Vec<char> = needle.chars().collect();
    let haystack: Vec<char> = haystack.chars().collect();
    'outer: for i in 0..(haystack.len() - needle.len() + 1) {
        for j in 0..needle.len() {
            if haystack[i + j] != needle[j] {
                continue 'outer;
            }
            println!("{} == {}", haystack[i + j], needle[j]);
        }
        return i as i32;
    }
    return -1;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_atoi_test() {
        let result = str_str(String::from("leetcode"), String::from("leeto"));
        assert_eq!(result, -1);
    }
}
