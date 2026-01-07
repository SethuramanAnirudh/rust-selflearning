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
    println!("Data types demo complete!");
}
