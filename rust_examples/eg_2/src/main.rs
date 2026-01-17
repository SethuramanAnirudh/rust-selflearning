#[test]
fn test_max_element() {
    assert_eq!(max_element(&[1,2,3,4,5,33]),Some(33));
    assert_eq!(max_element(&[]),None);
}
fn max_element(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut max = arr[0];
    let mut index = 1;
    while index < arr.len() {
        if arr[index] > max {
            max = arr[index];
        }
        index += 1;
    }
    Some(max)
}