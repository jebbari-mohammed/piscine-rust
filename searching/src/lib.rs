pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (i,&num) in array.iter().enumerate().rev() {
        match key == num {
            true => return Some(i),
            false => continue
        };
    };
    None
}