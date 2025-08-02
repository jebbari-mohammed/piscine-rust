pub fn num_to_ordinal(x: u32) -> String {
    let mut num = x.to_string() ;
    for ch in num.chars().rev().next() {
        match ch {
            '1' => num.push_str("st"),
            '2' => num.push_str("nd"),
            '3' => num.push_str("rd"),
             _ => num.push_str("th"),
        }
    };
    num
}