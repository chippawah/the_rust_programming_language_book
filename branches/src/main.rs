fn main() {
    let num = 3;
    // begins with the condition (an expression that must evaluate to a boolean) and the block of code to execute if it's true
    if num > 5 {
        // first arm of the if statement
        println!("Num is greater than 5 AKA condition is true");
    // optional handle for when the condition is not true
    // second arm of the if statement (and last because it uses the else keyword and not else if)
    // would be valid to omit this else arm if we didn't care about the false case of the condition
    } else {
        println!("Num is less than 5 AKA condition was false");
    }
    // handling multiple conditions with if else statements
    if num < 0 {
        println!("the num was negative")
    } else if num % 2 == 0 {
        println!("the number is even")
    } else {
        println!("the number was a non zero odd number")
    }

    // assigning variables to the evaluated value from the if statement. Note: the arms must return values of the same type
    // the below example uses an implicit return but the return keyword works here as well
    let iffy_num = if num > 1 { 0 } else { 1 };
    println!("The iffy_num is {iffy_num}") // should print out "The iffy_num is 0"

}
