pub fn is_empty(v: &str) -> bool {
    let mut bo = false;
    if v.len() == 0 {
       bo =  true;
    };
    bo

}

pub fn is_ascii(v: &str) -> bool {
    for ch in v.chars() {
        if ch < 'a' || ch > 'z' {
           return false
        };
    };
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    if v.contains(pat) {
        return true
    };
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    let char_pos = index;
    let (left, right) = v.split_at(char_pos);
    (left, right)
}

pub fn find(v: &str, pat: char) -> usize {
 v.find(pat).unwrap_or(999)
}
