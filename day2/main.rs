use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter number:");
    io::stdin()
    .read_line(&mut input)
    .expect("failed to read line");

    let number: i32 = input
    .trim()
    .parse()
    .expect("please enter valid numnber");

    println!("You entered: {}",number);
}