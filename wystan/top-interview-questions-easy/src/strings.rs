pub fn reverse_string(s: &mut Vec<char>) {
    let mut i: usize = 0;
    let mut j = s.len() - 1;
    while (j > i) {
        let mut tmp = s[i];
        s[i] = s[j];
        s[j] = tmp;
        i += 1;
        j -= 1;
    }
}
