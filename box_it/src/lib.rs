pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut boxx = Vec::new() ;
    let mut nums = s.split_whitespace();
    for num in nums {
        if num.ends_with("k") {
            let nb = &num[..num.len()-1];
            let mut n: f32 = nb.parse().unwrap();

            boxx.push(Box::new((n*1000.0)as u32))
        } else {
            let mut n: u32= num.parse().unwrap();
            boxx.push(Box::new(n))
        }
    }
    boxx
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut result = Vec::new();
    for num in a {
        result.push(*num);
    }
    result
}