fn main() {
    let vowel_count = count_vowels(&String::from("Anirudh Sethuraman"));
    println!("Number of vowels: {}", vowel_count);
    let digit_count = count_digits(&String::from("Ani678rudh1234"));
    println!("Number of digits: {}", digit_count);
}
fn count_vowels(s: &String) -> usize {
    let mut count = 0;
    for c in s.chars() {
        if c == 'a'
            || c == 'e'
            || c == 'i'
            || c == 'o'
            || c == 'u'
            || c == 'A'
            || c == 'E'
            || c == 'I'
            || c == 'O'
            || c == 'U'
        {
            count += 1;
        }
    }
    count
}

fn count_digits(s: &String) -> usize {
    let mut count = 0;
    for c in s.chars() {
        if c >= '0' && c <= '9' {
            count += 1;
        }
    }
    count
}
