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

    let original = String::from("Hello, Rust!");
    let reversed = reverse_string_rev(&original);
    println!("Original string: {}", original);
    println!("Reversed string: {}", reversed);

    let original2 = "Hello, World!";
    let reversed2 = reverse_string(original2);
    println!("Original string: {}", original2);
    println!("Reversed string: {}", reversed2);

    println!("{:?}", first_vowel("hello")); // Some('e')
    println!("{:?}", first_vowel("rhythm")); // None

    let name = "Anirudh";
    let greeting = greet(name);
    println!("{}", greeting); // "Hello, Anirudh!"
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
fn reverse_string_rev(s: &String) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}
fn reverse_string(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut i = chars.len();
    while i > 0 {
        i -= 1;
        result.push(chars[i]);
    }
    result
}
fn first_vowel(s: &str) -> Option<char> {
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
            return Some(c);
        }
    }
    None
}
fn greet(name: &str) -> String {
    let mut message = String::from("Hello, ");
    message.push_str(name);
    message.push('!');
    message
}
