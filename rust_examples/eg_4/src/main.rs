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
fn string_length(s: &String) -> usize {
    let mut count = 0;
    for _ in s.chars() {
        count += 1;
    }
    count
}
fn is_palindrome(s: &String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() == 0 {
        return true;
    }
    let mut left = 0;
    let mut right = chars.len() - 1;
    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
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

    let s1 = String::from("hello");
    assert_eq!(string_length(&s1), 5);
    let s2 = String::from("");
    assert_eq!(string_length(&s2), 0);
    let s3 = String::from("Anirudh Sethuraman");
    assert_eq!(string_length(&s3), 18);

    let p1 = String::from("racecar");
    assert_eq!(is_palindrome(&p1), true);
    let p2 = String::from("hello");
    assert_eq!(is_palindrome(&p2), false);
    let p3 = String::from("");
    assert_eq!(is_palindrome(&p3), true);
    let p4 = String::from("malayalam");
    assert_eq!(is_palindrome(&p4), true);
}
