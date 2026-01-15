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
    let s1 = gives_ownership();
    println!("the value of s1 is: {s1}");
    let s2 = String::from("hello from main");
    let s3 = s2;
    println!("the value of s3 is: {s3}");
    let s4 = takes_and_gives_back(s3);
    println!("the value of s4 is: {s4}");
}
fn takes_ownership(some_string: String) {
    println!("the value of some_string in takes_ownership is: {some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("the value of some_integer in makes_copy is: {some_integer}");
}
fn gives_ownership() -> String {
    let some_string = String::from("hello from gives_ownership");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    println!("the value of a_string in takes_and_gives_back is: {a_string}");
    a_string
}
