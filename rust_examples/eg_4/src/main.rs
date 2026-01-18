fn reverse_array_without_extra_memory(arr: &mut [i32]) {
    if arr.len() == 0 {
        return;
    }
    let mut start = 0;
    let mut end = arr.len() - 1;
    while start < end {
        let temp = arr[start];
        arr[start] = arr[end];
        arr[end] = temp;

        start += 1;
        end -= 1;
    }
}
#[test]
fn test_revenge() {
    let mut arr1 = [1, 2, 34, 5, 6, 4];
    reverse_array_without_extra_memory(&mut arr1);
    assert_eq!(arr1, [4, 6, 5, 34, 2, 1]);

    let mut arr2 = [];
    reverse_array_without_extra_memory(&mut arr2);
    assert_eq!(arr2, []);

    let mut arr3 = [1];
    reverse_array_without_extra_memory(&mut arr3);
    assert_eq!(arr3, [1]);

    let mut arr4 = [1, 2, 22, 33, 22, 33, 11, 44, 55];
    reverse_array_without_extra_memory(&mut arr4);
    assert_eq!(arr4, [55, 44, 11, 33, 22, 33, 22, 2, 1]);
}
