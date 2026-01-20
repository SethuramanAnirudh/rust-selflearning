fn main() {
    let vowel_count = count_vowels(&String::from("Anirudh Sethuraman"));
    println!("Number of vowels: {}", vowel_count);
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
