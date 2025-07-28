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
    let mut result = String::new();
    let mut word = String::new();
    let mut in_word = false;

    for ch in input.chars() {
        if ch.is_whitespace() {
            if in_word {
                result.push_str(&capitalize_first(&word));
                word.clear();
                in_word = false;
            }
            result.push(ch); 
        } else {
            word.push(ch);
            in_word = true;
        }
    }

    if in_word {
        result.push_str(&capitalize_first(&word));
    }

 
    result
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
