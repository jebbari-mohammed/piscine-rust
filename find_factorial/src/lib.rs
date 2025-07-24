pub fn factorial(num: u64) -> u64 {
    let mut fac = 1;
    for i in 1..=num {
        fac = fac*i
    };
    fac
}