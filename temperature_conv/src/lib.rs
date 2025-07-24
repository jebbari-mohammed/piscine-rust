pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let x = (f-32.0)*(5.0/9.0);
    (x * 100.0).round() / 100.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c*(9.0/5.0))+32.0
    
}