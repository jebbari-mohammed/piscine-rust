pub fn stars(n: u32) -> String {
    let mut number_of_stars = 1;
    let mut result = String::new();
    for _i in 1..=n {
        number_of_stars = number_of_stars * 2;
    }
    for _j in 1..=number_of_stars {
        result.push_str("*");
    };
    result.to_string()
}
