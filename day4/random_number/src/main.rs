use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    loop {
        println!("Guess the numnber between 1 and 100!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number");
                continue;
            }
        };
        let secret_no = rand::thread_rng().gen_range(1..=100);
        println!("the secret number is : {secret_no}");

        println!("You guessed : {guess}");
        match guess.cmp(&secret_no) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
