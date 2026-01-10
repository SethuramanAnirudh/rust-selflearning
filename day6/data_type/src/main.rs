fn main() {
    let _x = 5.0;
    let _y: f64 = 3.0;
    let _sum = 5 + 10;
    let _diff = 89.9 - 32.2;
    let _prod = 4 * 23;
    let _quotient = 56.7 / 32.2;
    let _trunc = -5 / 3;
    let _remainder = 43 % 5;
    // if not used, prefix variable name with _ to avoid warnings

    let _t = true;
    let _f: bool = false;
    /*
        let t = true;
      |         ^ help: if this is intentional, prefix it with an underscore: `_t`
    */

    println!("_x = {}, _f(boolen) = {}", _x, _f);
    println!(
        "Integer operations: {},{},{},{},{},{}",
        _sum, _diff, _prod, _quotient, _trunc, _remainder
    );
    let c = 'A';
    let s: char = 'w';
    println!("Characters: {}, {}", c, s);

    let string: &str = "Hello,Rust!";
    println!("String {}", string);
    let tup: (i32, f64, u8) = (11, 3.14, 255);
    let (a, b, c) = tup;
    println!("Tuple elements: {}, {}, {}", a, b, c);
    let first_element = tup.0;
    println!("First element of tuple: {}", first_element); // should print 11
    let arr: [i32; 6] = [1, 2, 3, 40, 5, 6];
    let first_array_ele = arr[0];
    println!("First element of array: {}", first_array_ele); // should print 1
    println!("Fourth element of array: {}", arr[3]); // should print 40

    println!("Enter a array index to access:");
    let mut index_input = String::new();
    std::io::stdin()
        .read_line(&mut index_input)
        .expect("Failed to read line");
    // let index: usize = index_input
    //     .trim()
    //     .parse()
    //     .expect("please enter a numerical number");
    let index: usize = match index_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };
    match arr.get(index) {
        Some(element) => {
            println!("the value of element at index {} is: {}", index, element);
        }
        None => {
            println!(
                "Index out of bounds! valid indices are from 0 to {}",
                arr.len() - 1
            );
            return;
        }
    }
    let element = arr[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
    println!("Data types demo complete!");
}
