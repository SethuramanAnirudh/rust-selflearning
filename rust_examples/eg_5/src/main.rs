fn main() {
    let vowel_count = count_vowels(&String::from("Anirudh Sethuraman"));
    println!("Number of vowels: {}", vowel_count);
    let digit_count = count_digits(&String::from("Ani678rudh1234"));
    println!("Number of digits: {}", digit_count);
    let only_digits = is_only_digits(&String::from("1234567890"));
    println!("Is only digits: {}", only_digits);
    let not_only_digits = is_only_digits(&String::from("Ani12345"));
    println!("Is only digits: {}", not_only_digits);

    let word_count = count_words(&String::from("Hello world this is Rust"));
    println!("Number of words: {}", word_count);

    let word_count2 = count_words(&String::from("   Leading and trailing spaces   "));
    println!("Number of words: {}", word_count2);

    let word_count3 = count_words(&String::from(""));
    println!("Number of words: {}", word_count3);

    let word_count4 = count_words(&String::from("SingleWord"));
    println!("Number of words: {}", word_count4);
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

fn is_only_digits(s: &String) -> bool {
    for c in s.chars() {
        if c < '0' || c > '9' {
            return false;
        }
    }
    return true;
}
fn count_words(s: &String) -> usize {
    let mut count = 0;
    let mut in_word = false;
    for c in s.chars() {
        if c != ' ' && !in_word {
            count += 1;
            in_word = true;
        } else if c == ' ' {
            in_word = false;
        }
    }
    count
}
