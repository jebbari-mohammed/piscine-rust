use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut map = HashMap::new();
    for ch in s1.chars() {
        map.insert(ch,false);
    }
    for ch in s2.chars() {
        match map.get(&ch) {
            Some(_) => map.insert(ch,true),
            None => map.insert(ch,false),
        };
    }
    for (_key, value) in map {
        if value == false {
            return false
        };
    }
    true
}