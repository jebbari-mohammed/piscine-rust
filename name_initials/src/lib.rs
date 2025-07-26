pub fn initials(names: Vec<&str>) -> Vec<String>  {
    let mut res: Vec<String> = vec![];
    
    for name in names {
        let mut txt = String::new();

        
        for charr in name.split(' '){
            let first = charr.chars().next().unwrap();
            txt.push(first);
            txt.push_str(". ");
            // println!("{}",txt);
            // res.push(first.to_string());
            // res.push(".".to_string())
        };
        txt.pop();
        res.push(txt);
    };
    res
}
// pub fn formating(s : String ) -> String {
//     println!("===>{}",s);
//     s
// }