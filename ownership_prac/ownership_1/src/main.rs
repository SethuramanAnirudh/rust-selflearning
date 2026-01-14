fn main() {
    let x = 5;
    // let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x inside the inner scope is: {x}");
    }
    println!("the value of x is: {x}");
    let s = String::from("hello");
    takes_ownership(s);
    let x = 10;
    makes_copy(x);
    println!("the value of x after makes_copy is: {x}");
}
fn takes_ownership(some_string: String) {
    println!("the value of some_string in takes_ownership is: {some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("the value of some_integer in makes_copy is: {some_integer}");
}
