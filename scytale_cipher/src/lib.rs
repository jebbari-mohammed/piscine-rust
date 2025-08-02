pub fn scytale_cipher(message: &str, i: usize) -> String {
    let mut first = vec![];
    let mut second = vec![];
    let mut result = String::new();
    for (j, ch) in message.chars().enumerate() {
        if j < i {
            first.push(ch);
        } else {
            second.push(ch);
        }
    }
    let diff = first.len() - second.len() ;
    for _j in 0..diff {
        second.push(' ');
    }
    // println!("{:?}",first);
    // println!("{:?}",second);

    let len = first.len();
    for j in 0..len {
            result.push(first[j]);
            result.push(second[j]);
    }

    result = result.trim().to_string();
    result
}
