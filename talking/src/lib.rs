pub fn talking(text: &str) -> &str {
    if text.trim() == "" {
        return "Just say something!";
    }
    if everything_is_uppercase(text) {
        if text[text.len() - 1..].to_string() == "?" {
            return "Quiet, I am thinking!";
        } else {
            return "There is no need to yell, calm down!";
        }
    } else {
        if text[text.len() - 1..].to_string() == "?" {
            return "Sure.";
        } else {
            return "Interesting";
        }
    }
}
pub fn everything_is_uppercase(s: &str) -> bool {
    if !s.chars().all(|c| c.is_alphabetic()) {
        return false;
    }
    for ch in s.chars() {
        if ch.is_lowercase() {
            return false;
        }
    }
    true
}
