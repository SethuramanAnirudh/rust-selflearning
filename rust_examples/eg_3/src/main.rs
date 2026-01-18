fn count_even_odd(arr: &[i32]) -> (usize, usize) {
    let mut even_count = 0;
    let mut odd_count = 0;
    let mut index = 0;
    while index < arr.len() {
        if arr[index] % 2 == 0 {
            even_count += 1;
        } else {
            odd_count += 1;
        }
        index += 1;
    }
    (even_count, odd_count)
}
fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 22, 34, 45, 323, 433, 2324];
    let (even, odd) = count_even_odd(&arr);
    println!("Even count: {}, Odd count: {}", even, odd);
}
