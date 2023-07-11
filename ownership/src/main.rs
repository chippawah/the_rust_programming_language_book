fn main() {
    scoping_example();
    fun_with_strings();
    function_fun();
    reference_fun();
    mutable_reference_fun();
    print_reference_rules();
}

fn scoping_example() {
    println!("\n--- SCOPING EXAMPLE ---");
    // Here x is invalid as it hasn't been declared
    let x = 10;
    println!("x is in the scope of scoping_example and equals: {x}");
    let y = x;
    println!("Unlike the clone and move examples below x will not be invalidated because it's value is known at runtime and implements the Copy trait.");
    println!("x: {x}, y: {y}");
    // x is dropped at the end of the fn, much like other programming languages
}

// String data is stored on the heap and the variable on the stack is a pointer to the heap data
// This memory is allocated at *runtime* which is often neccessary but slightly less performant
// Any variables defined here are dropped from memory at the end of the fn
fn fun_with_strings() {
    println!("\n--- STRING OWNERSHIP EXAMPLE ---");
    // Though this string is a known size now it is still allocated on the heap so it can grow and shrink
    let s = String::from("Hello World");
    println!("The String type is allocated on the heap. The s variable is a pointer on the stack that points to heap data equal to: {s}");
    
    let mut a_mutable_string = String::from("Hello, ");
    println!("A mutable string before mutation: {a_mutable_string}");
    a_mutable_string.push_str("World!");
    println!("A mutable string after growing in size: {a_mutable_string}");
    // This automatic deallocation is sometimes called RAII (Resource Acquisition Is Initialization) and found in C++
    
    // The s variable from above contains a pointer to heap data.
    // The pointer is a length, a capacity, and the point in memory on the heap where the data is
    let s1 = s;// s goes out of scope here solving the double free error that may occur if s and s1 go out of scope at the same time
    println!("The data in the heap that s pointed to has been *moved* to s1 and the s variable (with its pointer to the heap data) gets invalidated. s1: {s1}");

    // If we a copy of s1 without invalidating it we can call clone()
    // Clone copies the heap data instead of just *moving* s1's heap data to the s2 variable
    // Be warned that cloning is more expensive than just moving data
    let s2 = s1.clone();
    println!("fun_with_strings's s2 is a clone of the s1 string so both are still valid and their heap data still has only one owner");
    println!("fun_with_strings's s1: {s1} | s2: {s2}");
}

fn function_fun() {
    println!("\n--- FUNCTION OWNERSHIP EXAMPLES ---");
    let s = String::from("I'll live another day!");
    // s is valid to use in this until something else takes ownership or the scope ends
    println!("The string in function_fun before it's been moved: {s}");
    takes_ownership(s);
    // s is no longer a valid reference once it has been passed into the takes_ownership function
    // Using s after this point would result in a compile time error
    let x = 4;
    println!("The data bound to the variable x will be copied: {x}");
    makes_copy(x);
    println!("The variable x is still valid after being used as a function argument for makes_copy: {x}");
    // Once the function_fun scope ends x is dropped. s was already dropped when it was used as an argument to takes_ownership
    
    // Although the String that s1 is bound to was created in another scope the function_fun's s1 owns the data once it's returned
    let s1 = gives_ownership();
    println!("function_fun recieved ownership of a value from gives_ownership: {s1}");
    // s1 gets dropped when the function_fun scope

    let s2 = String::from("Hello, World!");
    println!("function_fun's s2 variable owns this string: {s2}");
    // Once s2 is passed as an argument to a function it's invalidated and can't be used again
    let s3 = gives_and_takes_ownership(s1);
    println!("function_fun's s3 now owns the string that s2 used to. s3: {s3}");
    // s3 goes out of scope and is dropped at the end of function_fun's scope
}

// This fn will take ownership of the argument
fn takes_ownership(x: String) {
    println!("The string this fn owns is: {x}");
    // x goes out of scope here and is dropped from memory
}

// This fn will only make a copy because the integer type impletments the Copy trait
fn makes_copy(x: i32) {
    println!("The integer this fn copies is: {x}")
    // x goes out of scope and the locally scoped x is dropped from memory
}

// the value returned from this function will be owned by the calling scope
fn gives_ownership() -> String {
    let s = String::from("I'm gonna move to the caller's scope");
    println!("The string I'm going to return and give ownership for is: {s}");
    return s
}

// this fn will own the underlying data for it's argument and then return the value to the caller and give ownership of the value to the caller
// worth noting that taking and returning ownership isn't very idiomatic
fn gives_and_takes_ownership(s: String) -> String {
    println!("gives_and_takes_onwership received and owns: {s}");
    return s;
}

fn reference_fun() {
    println!("\n--- REFERENCE AND BORROWING EXAMPLES ---");
    // Say we want to calculate the length of a string and print that string and it's length after calculation.
    // In the example below we could avoid using references like so:
    let s = String::from("Hello, World!");
    let (s2, length) = tedious_len_calc(s);
    println!("reference_fun | shuffled ownership string's length: {s2} {length}");
    let length = referenced_len_calc(&s2); // The s2 variable is borrowed here using a reference.
    println!("reference_fun | referenced string's length: {s2} {length}");

    let r = &s2;
    let r1 = &s2;
    println!("multiple immutable references are allowed at once. Both r and r1 are immutable references to s2. r: {r} r1: {r1}");
}

// This could become tedious to move ownership around like this and return tuples
// the normal ownership and scoping rules still apply
fn tedious_len_calc(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len);
}

fn referenced_len_calc(s: &String) -> usize {
    // here s is just a pointer to the stack value that in turn points to heap data containing the string
    return s.len();
    // s goes out of scope here but the String does not get dropped because it was just being borrowed
}

fn mutable_reference_fun() {
    println!("\n--- MUTABLE REFERENCE EXAMPLES ---");
    let mut s = String::from("Hello, ");
    
    // this won't work the borrowed reference hasn't been declared mutable
    // let r = &s;
    // r.push_str("This won't work");
    
    // The following works because the borrowed reference is mutable and the compiler can infer that r is also mutable
    let r = &mut s;
    r.push_str("World");
    println!("s after r borrowed and mutated it: {s}");
    // The following doesn't work because you can only have one mutable borrow at a time a r is it.
    // s.push_str("!");
    println!("Mutiple immutable references are allowed *before* a mutable reference is declared if they aren't used again within scope");
    let mut s = String::from("Hello, ");
    println!("Shadowed s is mutable and set to: {s}");
    
    let r = &s;
    let r1 = &s;
    println!("shadowed r: '{r}' and r1: '{r1}' will not be used again in this scope and therefore their scope ends");
    
    let r3 = &mut s;
    r3.push_str("World!");
    println!("r3 is a mutable reference created after the scope for r1 and r2 ended. r3: '{r3}'");
}

fn print_reference_rules(){
    println!("\nReference rules:");
    println!("  1. Either one mutable reference is allowed at any given time or multiple immutable references but not both.");
    println!("  4. References must be valid. (no dangling)");
}
// Below is an example of code that won't compile because it attempts to dangle a reference:
// fn dangler() -> &String {
//     let s = String::new;
//     &s
// }