pub fn str_len<T: AsRef<str>>(s: T) -> usize {
    s.as_ref().chars().count()
}