pub fn get_diamond(c: char) -> Vec<String> {
    let alphabet = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let mut res = Vec::new();
    let mut finalres = Vec::new();
    let idx = (c as u8 - b'A') as usize;

    for i in 0..=idx {
        let mut line = String::new();
        let out_spaces = idx - i;
        let middle_spaces = if i == 0 { 0 } else { 2 * i - 1 };

        line.push_str(&make_spaces(out_spaces));
        line.push(alphabet[i]);
        if middle_spaces > 0 {
            line.push_str(&make_spaces(middle_spaces));
            line.push(alphabet[i]);
        }
        line.push_str(&make_spaces(out_spaces));
        res.push(line);
    }

    finalres.extend(res.iter().cloned());

    for line in res.iter().rev().skip(1) {
        finalres.push(line.clone());
    }

    finalres
}

fn make_spaces(n: usize) -> String {
    " ".repeat(n)
}
