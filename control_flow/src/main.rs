fn main() {
    // Control Flow
    // ---

    // `if` Expressions
    // ----
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //  The condition in the `if` must be a bool.
    //  IF the condiiton is not a bool an error will be thrown.
    //  This is unlike other languages that will corece the code
    //  to be a bool.

    // Handling Multiple Conditions with `else if`
    // ---
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using too many `else if` can clutter the code.
    // Opt for a `match` in these cases.

    // Using `if` in a `let` Statement
    // ---
    // Because if is an expression, we can use it
    // on the right side of a let statement to assign
    // the outcome to a variable
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // They types returend from each arm of the should be
    // the same type

    // Repetions with Loops
    // ---

    // Repeating Code with `loop`
    // ---
    // The `loop` keyword executes a code block
    // over and over again until explicitly stopped.
    loop {
        let count = 0;
        if count == 1 {
            continue; // skips this iteration of the loop
        }
        println!("again");

        if count > 3 {
            break; // exits loop
        }
    }

    // Returning Values from Loops
    // ---
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // add value you want returned 
            // after the break expression;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    // ---
    // With nested loops, the `break` and `continue` expressions
    // by default apply to the innermost loop at that point.
    // Optionally a loop label can be used non a loop and
    // thenn used with either `break` or `continue` to specify
    // a specific loop.
    let mut count = 0;
    'counting_up: loop {
        // labeled loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // break the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with `while`
    // ---
    // A handy construct provided by Rust that allows
    // to loop until a certain condition is met.
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping Through a Collection with `for`
    // ---
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
