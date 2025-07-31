const CONSTANT_VARUABLE: u32 = 1;

fn main() {
    println!("This is the constant variable: {CONSTANT_VARUABLE}");
    let y = 10; // immutable variable. Can not be changed.

    let mut x = 5; // mutable var. Can be changed after creation.

    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("This is an immutable variable: {y}")
}
