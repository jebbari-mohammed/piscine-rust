pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let c_f64 = c as f64;
    let t: (i32, f64, f64) = (c, c_f64.exp(), c_f64.abs().ln());
    t
}

pub fn str_function(a: String) -> (String, String) {
    let mut exp_values = Vec::new();

    for ch in a.chars() {
        if ch == ' ' {
            continue;
        }
        if let Some(digit) = ch.to_digit(10) {
            let val = digit as f64;
            let exp_val = val.exp();
            exp_values.push(exp_val.to_string());
        }
    }
    let exp_str = exp_values.join(" ");
    (a, exp_str)
}



pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut exp_values = Vec::new();

    for &num in &b {
        let val = num as f64;
        let exp_val = val.ln();
        exp_values.push(exp_val);
    }

    (b, exp_values)
}
