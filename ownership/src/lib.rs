pub fn first_subword(mut s: String) -> String {
    let mut index = 0;
    if s.contains("_"){
        for cha in s.chars() {
            if cha == '_'{
                break
            };
            index += 1;
        };
        return s[0..index].to_string()
    } else {
        for cha in s[1..].to_string().chars() {
            if cha.is_uppercase() {
                break
            };
            index += 1;
        };
        return s[0..index+1].to_string()
    };
}