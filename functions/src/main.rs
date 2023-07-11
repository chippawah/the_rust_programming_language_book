// Notes about functions in general:
// They can optionally accept arguments by having parameters defined when the fn is creataed and can optionally return a value
// Functions are made of a series of statements followed optionally by an expression.
// Statements are instructions that perform an action and *do not* return a value
// Expressions evaluate to a value.
// Function declarations are statements;
// Rust is an expression-based language

fn main() {
    another_function();
    parameterized_fn(5);
    print_val_and_unit(10, "inches");
    statement_shenanigans();
    let implicit_return = returns_five_implicitly();
    println!("Implicitly returned: {implicit_return}");
    let explicit_return = returns_five_explicitly();
    println!("Explicitly returned: {explicit_return}")
}

// Function without parameters
fn another_function() {
    println!("Hello, world!");
}
// Fn with parameters. Parameter types *must* be indicated.
fn parameterized_fn(x: i32) {
    println!("This is a fn that has parameters. The given arguements are x: {x}");
}
// Fn that takes in two differently typed parameters
fn print_val_and_unit(val: i32, unit: &str) {
    println!("The measurement is: {val} {unit}");
}

// Statement practice
fn statement_shenanigans() {
    // valid statement
    let x = 10;
    // Invalid statement because the = sign must be followed by an expression;
    // let x = (let y = 10);

    // statements can contain expressions like the following:
    let y = {
        let z = 5;
        x + z // Note that theres no semicolon which is an implicit return
    };
    println!("statement shenaninigans: y is {y}")
}

// Below are examples of returning a value both implicitly and explicitly
fn returns_five_implicitly() -> u8 {
    5
}

fn returns_five_explicitly() -> u8 {
    return 5;
}
