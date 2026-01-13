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
        println!("the value is:{}", a[index]);
        index += 1;
    }
}
fn while_array_example_rev() {
    let a = [10, 20, 30, 40, 50];
    let mut index = a.len();
    while index > 0 {
        index -= 1;
        println!("while_array_example_rev the value is:{}", a[index]);
    }
}
