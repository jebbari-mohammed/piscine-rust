pub fn scytale_cipher(message: &str, i: usize) -> String {
    let mut first = vec![];
    let mut second = vec![];
    let mut third = vec![];
    let mut result = String::new();
    let mut nosec = false;
    let mut noth = false;
    for (j, ch) in message.chars().enumerate() {
        if j < i {
            first.push(ch);
        } if j>=i && j<i*2 {
            second.push(ch);
        }
        if j>=i*2 && j<i*3 {
            third.push(ch);
        }
    }
    let mut diff = first.len().saturating_sub(second.len());
    if diff == 0 {
        diff = second.len().saturating_sub(third.len());
    }
    println!("{}",diff);
    for _j in 0..diff {
        if first.len() > second.len() {
            second.push(' ');
        }
        if second.len() > third.len() {
            third.push(' ');
        }
    }
    println!("{:?}",first);
    println!("{:?}",second);
    println!("{:?}",third);
    nosec = only_spaces(second.clone());
    noth = only_spaces(third.clone());

    let len = first.len();
    for j in 0..len {
            result.push(first[j]);
            if !nosec {
                result.push(second[j]);
            }
            if !noth {
                result.push(third[j]);
            }
    }

    result.trim().to_string()
}
pub fn only_spaces(v : Vec<char>) -> bool{
    for ch in v {
        if ch != ' '{
            return false
        };
    }
    true
}