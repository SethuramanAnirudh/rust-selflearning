// fn main() {
//     // println!("Hello, world!");
//     test_sum_array();
// }
#[test]
fn test_sum_array() {
    assert_eq!(sum_array(&[1,2,3,4,5,6]),21);
    assert_eq!(sum_array(&[]),0);
    assert_eq!(sum_array(&[-1,1,-1,1]),0);
}
fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    let mut index = 0;
    while index < arr.len() {
        sum += arr[index];
        index += 1;
    }
    sum
}
