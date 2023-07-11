fn main() {
    // Mutability
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");
    
    // Scoping:
    let x = 5; 
    let x = x + 1;
    {
        let x = x * 2;
        println!("Inner scoped x {x}");
    }
    println!("Outer scoped x {x}");
    
    // Scalar Types: Booleans, Integers, Floating Point Numbers, and Characters (Unicode Scalar)
    // Compound Types: Arrays and Tuples
    // Tuples can be composed of multiple types but must remain the same size.
    let tup: (u8, u32, char) = (1, 12, 'f');
    // Tuples can be pattern destructured:
    let (my_eight_bit_int, my_thirty_two_bit_int, my_char) = tup;
    println!("{my_eight_bit_int}, {my_thirty_two_bit_int}, {my_char}");
    // You can also use indexing which starts at 0:
    let my_eight_bit_int = tup.0;
    let my_thirty_two_bit_int = tup.1;
    let my_char = tup.2;
    println!("{my_eight_bit_int}, {my_thirty_two_bit_int}, {my_char}");
    // Arrays cannot change size and must be of the same type they get allocated on the stack
    // Useful when you know the length of a collection will remain the same like the months of the year.
    let months: [&str; 12] = [
        "Jan",
        "Feb",
        "Mar",
        "Apr",
        "May",
        "Jun",
        "Jul",
        "Aug",
        "Sep",
        "Oct",
        "Nov",
        "Dec"
    ];
    let june = months[5];
    println!("the variable name june is assined to months[5]. june = {june}");
    // Accessing arrays with an invalid index will cause runtime panic.
    // Invalid: let x = [1,2,3,4]; let some_num = x[1000000];
    
}
