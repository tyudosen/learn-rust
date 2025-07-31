// Functions
// ----
// Functions in Rust start with an `fn` keyword
// followed by the name of the function and a set of paranthesis

// Rust Entry Point
fn main() {
    println!("main function");

    // use of declared function
    another_function();

    function_with_param(10);
    function_with_params(32, 'm');
    let x = five();

    println!("The value returned from five: {x}");

    let x = plus_one(5);
    println!("The value returned from plus_one: {x}");

}

// Function Declaration
// -----
// Rust does not care where the function is declared.
// As long as the function is in scope of the caller.
fn another_function() {
    println!("Another function");
}

// Parameters
// -----
// function signatures must declare the types of their
// parameters
fn function_with_param(x: i32) {
    println!("This is the value of the passed param: {x}");
}

fn function_with_params(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements and Expressions
// -----
// **Statements** are instructions that perform actions
// and do not return a value;
//
// **Expressions** evaluate to a resultant value.

fn statements_and_expressions() {

    let y = 6; // statement
    // Statements do not return a value..
    // Hence they can not be used to assign to another
    // variable;
    // A function definitions are statements as well.

    let x = (let y = 6); // Error.

    /* Expressions return results.
     The value on teh right side of a let statement
     for example `6` in `let y = 6` is an expression.
     Calling a function, calling a macro, new scope blocks
     created with curly braces are all expressions */
    let y = {
        let x = 3;
        x + 1 // note this does not end with a semicolon.
        // Expressions do not end with a semicolon.
        // A semicolon will turn it into a statement.
    };
}

// Functions with Return Values
// ----
/* Functions can return values.
 You must annotate the return type.*/
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}




