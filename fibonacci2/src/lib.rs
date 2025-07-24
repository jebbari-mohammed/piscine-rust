pub fn fibonacci(n: u32) -> u32 {
if n == 0 {
    return 0
}
if n == 1 {
    return 1
}
fibonacci(n-1) + fibonacci(n-2)
}