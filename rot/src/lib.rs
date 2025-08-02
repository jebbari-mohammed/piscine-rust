pub fn rotate(input: &str, key: i8) -> String {
    let alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut result = String::new();

    for ch in input.chars() {
        let lower = ch.to_ascii_lowercase();

        if lower >= 'a' && lower <= 'z' {
            let idx = (lower as i8 - b'a' as i8) as i8;
            let new_idx = (idx + key).rem_euclid(26) as usize;
            let rotated_char = alphabet[new_idx];

            if ch.is_ascii_uppercase() {
                result.push(rotated_char.to_ascii_uppercase());
            } else {
                result.push(rotated_char);
            }
        } else {
            result.push(ch);
        }
    }

    result
}
