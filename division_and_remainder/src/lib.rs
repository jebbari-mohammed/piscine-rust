pub fn divide(x: i32, y: i32) -> (i32, i32) {
        let tup = (x/y, x%y);

    let (_division, _remainder) = tup;
    tup
}