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
            let x = i.split('+');
            *i = sum(x[0],x[1]);
        } else if i.contains('-') {
            *i = sub(i.clone());
        }
    }
}
