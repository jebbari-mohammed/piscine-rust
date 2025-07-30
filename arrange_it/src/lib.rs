pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split_whitespace().collect();
    let lenght = words.len();
    let mut strr = String::new();
    let mut result = String::new();
    let mut num = 1;
    for i in 0..lenght {
        for word in &words {
            if word.contains(&num.to_string()) {
            for charr in word.chars() {
            if Some(charr) == std::char::from_digit(num, 10) {
                 num += 1;
            continue;
            }
                strr.push(charr);
            };
            strr.push(' ');
            result.push_str(&strr);
            strr = String::new();
        }
        };
    }; 
    result
}