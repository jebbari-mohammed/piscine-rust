pub fn to_url(s: &str) -> String {
    let mut text = String::new();
    for ch in s.chars() {
        if ch == ' '{
            text.push_str("%20");
            continue
        }
        text.push(ch);
    };
    text
}