pub fn rev_str(input: &str) -> String {
    let mut text = String::new();
    for cr in input.chars().rev(){
        text.push(cr)
    }
    text
}