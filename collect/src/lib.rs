pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for j in 0..len {
        for i in 0..len - j - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
    }
}
