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
