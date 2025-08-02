pub fn pig_latin(mut text: &str) -> String {
    let mut result = String::new();
    let mut mmove = String::new();
    let voils = "aeiou";
    let mut finish = false;
    let mut q = false;
    // les mut u = false;
    let first_char = &text[0..1];
    if voils.contains(&first_char) {
        result.push_str(text);
        result.push_str("ay");
        return result;
    } else {
        mmove.push_str(first_char);
        text = &text[1..];
    }
    for ch in text.chars() {
        if finish {
            result.push_str(&ch.to_string());
            continue;
        }
        if q && ch == 'u' {
            finish = true;
            mmove.push_str(&ch.to_string());
            continue
        }
        if ch == 'q' {
            q = true;
            mmove.push_str(&ch.to_string());
            continue
        }
        if voils.contains(ch) {
            if !q {
                result.push_str(&ch.to_string());
                finish = true;
                continue;
            }
        }
        q = false;
        mmove.push_str(&ch.to_string());
    }
    mmove.push_str("ay");
    result.push_str(&mmove);
    result
}
