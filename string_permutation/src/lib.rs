use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut map = HashMap::new();

    for ch in s1.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }
    for ch in s2.chars() {
        match map.get_mut(&ch) {
            Some(count) => {
                if *count == 0 {
                    return false;
                }
                *count -= 1;
            }
            None => {
                return false;
            }
        }
    }
    for &count in map.values() {
        if count != 0 {
            return false;
        }
    }
    true
}
