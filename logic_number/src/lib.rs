pub fn number_logic(num: u32) -> bool {
    let strnum = num.to_string();
    let mut result = 0 ;
    if strnum.len() == 1 {
        return true
    };
    for n in strnum.chars(){
        let numm = n.to_string().parse().unwrap();
        result += power(numm,strnum.len());
    }
    if result == num {
        return true
    };
    false

}
pub fn power(n : u32 , pow : usize) -> u32 {
    let mut res = 1 ;
    for _i in 0..pow {
        res = res*n;
    }
    res
}
