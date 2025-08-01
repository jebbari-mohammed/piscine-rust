pub fn delete_and_backspace(s: &mut String) { // hel-lo
    let mut v = vec![];
    let mut boo = 0;
    let mut result = String::new();
    for i in s.chars() {
        if i=='-' {
            v.pop();
            continue;
        }
        if i=='+' {
            boo = boo+1;
            continue;
        }
        if boo == 0{
            v.push(i);

        } else {

            boo = boo-1;
        }
    };
        for i in v {
        result.push(i);
    };
    *s = result
}

pub fn do_operations(v: &mut [String]) {
    for i in v.iter_mut() {
        if i.contains('+') {
            let parts : Vec<&str> = i.split('+').collect();

                *i = sum(parts[0], parts[1]);
            
        } else if i.contains('-') {
            let parts: Vec<&str> = i.split('-').collect();
            if parts.len() == 2 {
                *i = sub(parts[0], parts[1]);
            }
        }
    }
}

pub fn sum(x: &str, y: &str) -> String {
    let a: i32 = x.trim().parse().unwrap();
    let b: i32 = y.trim().parse().unwrap();
    (a + b).to_string()
}

pub fn sub(x: &str, y: &str) -> String {
    let a: i32 = x.trim().parse().unwrap();
    let b: i32 = y.trim().parse().unwrap();
    (a - b).to_string()
}
