fn main() {
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;
        loop {
            println!("count = {count}, remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    while_example();
    while_array_example();
    while_array_example_rev();
    for_array_example();
    for_range_example();
}
fn while_example() {
    let mut num = 5;
    while num != 0 {
        println!("{num}!!");
        num -= 1;
    }
    println!("wowwww");
}
fn while_array_example() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("while_array_example ==> the value is:{}", a[index]);
        index += 1;
    }
}
fn while_array_example_rev() {
    let a = [10, 20, 30, 40, 50];
    let mut index = a.len();
    while index > 0 {
        index -= 1;
        println!("while_array_example_rev ==> the value is:{}", a[index]);
    }
}
fn for_array_example() {
    let a = [100, 200, 300, 400, 500];
    for element in a {
        println!("for_array_example ==> the value is:{}", element);
    }
}
fn for_range_example() {
    for number in (1..10).rev() {
        println!("for_range_example ==> the value is: {}", number);
    }
    println!("LIFTOFF!!!");
}
