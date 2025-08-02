pub fn num_to_ordinal(x: u32) -> String {
    let mut num = x.to_string() ;
    let case = &num[(num.len()-2)..num.len()];
    let x: u32 = case.parse().unwrap();
    let res = String::new();
    if x <= 19 && x >= 10 {
        num.push_str("th");
        return num;
    }
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