fn main() {
    println!("In main function");
    another_function(5);
    print_labled_value(10, 'A');
    let x = five();
    println!("The value returned from five() is {x}");
    println!("Enter a number to add 2 to it:");
    let mut a = String::new();
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    let sum = add(a.trim().parse().expect("number"), 2);
    println!("The sum retured from add() is {sum}");
    control_statement(sum);
    let mut condition = true;
    let true_number = if condition { 100 } else { 1000 };
    println!("The value of number if conditon true is: {true_number}");
    condition = false;
    let false_number = if condition { 100 } else { 1000 };
    println!("The value of number if condition false  is: {false_number}");
    // _loop_example_bug(sum);
    loop_example(sum);
}
fn another_function(x: i32) {
    println!("In another function {x}");
}
fn print_labled_value(value: i32, unit_label: char) {
    println!("The value is {} {}", value, unit_label);
}
fn five() -> i32 {
    5
}
fn add(x: i32, y: i32) -> i32 {
    x + y
}
fn control_statement(x: i32) {
    if x < 10 {
        println!("Condition was true, {x} is less than 10");
    } else if x == 10 {
        println!("{x} is equal than 10");
    } else {
        println!("Condition was false, {x} is greater than 10");
    }
}
// this function has a bug, if x is greater than 10, it will loop infinitely
// This is exactly the kind of bug Rust/ or any programming language
// does NOT protect you from — it’s logical, not memory-related.
fn _loop_example_bug(x: i32) {
    let mut count = x;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is {result}");
}
fn loop_example(x: i32) {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == x {
            break count * 2;
        }
    };
    println!("The result is {result}");
}
