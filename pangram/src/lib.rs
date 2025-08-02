pub fn is_pangram(s: &str) -> bool {
    let mut seen = [false; 26];
    for ch in s.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower <= 'z' && lower >= 'a'  {
            let idx = (lower as u8 - b'a') as usize;
            seen[idx] = true;
        }
    }
    for &value in &seen {
        if value == false {
            return false
        };
    }
    return true
}
