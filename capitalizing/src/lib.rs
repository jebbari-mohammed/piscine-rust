pub fn capitalize_first(input: &str) -> String {
    let mut index = 0;
    let mut res = String::new();
    for ch in input.chars() {
        if index == 0 {
            index += 1;
            res.push_str(&ch.to_string().to_uppercase());
            continue;
        };
        res.push(ch)
    };
    res
}

pub fn title_case(input: &str) -> String {
    let mut strr = String::new();
    for ch in input.split_whitespace() {
        strr.push_str(&capitalize_first(ch));
        strr.push(' ')
    };
    strr
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();

    for ch in input.chars() {
        if ch.is_lowercase() {
            result.extend(ch.to_uppercase());
        } else if ch.is_uppercase() {
            result.extend(ch.to_lowercase());
        } else {
            result.push(ch);
        }
    }

    result
}
