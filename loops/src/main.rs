// General loopy notes:
// Three types of loops: "loop": infinite, "while": do something while a condition is true, "for": loops until a condition is met (with some extra syntax sugar)
// loops created with the loop keyword go infinitely until the program is killed or the loop encounters a break statement
// A break statement will skip the rest of the code in the loop and exit it, returning execution the the next line after the loop block
// Unlike the break statement a continue statement will skip everything remaining in the loop block and go to the next iteration
// break and continue statements only apply to the inner most scope they are in unless labeled.

fn main() {
    // _infinite_loop();
    returning_a_val_from_a_loop();
    loopy_labeled_loops_in_loops();
    wild_while_loop();
    wily_while_looping_array();
    wilier_array_while_loop();
    rustier_countdown_loop();
}

// undersxcored to ignore unused code. Commented out because we don't want an infinite loop running
// could also just call this fn last in main if we don't want it to block the others
fn _infinite_loop() {
    loop {
        println!("Hello, world!");
    }
}

fn returning_a_val_from_a_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // kinda funky syntax here but basically just saying the result should be set to the expression following the break statement
        }
    };
    println!("The result variable has been set to {result}"); // should print out "The result variable has been set to 20"
}

// The example below shows how loop labels can be applied
// labels must begin with a single quote mark and be followed by a colon
fn loopy_labeled_loops_in_loops() {
    let mut count = 0;
    'counting_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // only breaks of out the inner unlabeled loop
            }
            if count == 2 {
                break 'counting_loop; // specifying the labeled loop name we can break out of the specified loop.
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Final count: {count}");
}

// the while loop executed until the condition its given ceases to be true
fn wild_while_loop() {
    let mut num = 3;
    while num != 0 {
        println!("{num}...");
        num -= 1;
    }
    println!("LIFTOFF!");
}

fn wily_while_looping_array() {
    let a = [1,2,3,4,5];
    let mut index = 0;
    // not very idomatic way of doing things using a.len() there if the next example is to be considered better
    while index < a.len() {
        println!("The value of a[{index}] is {}", a[index]);
        index += 1;
    }
}

// for in loop
fn wilier_array_while_loop() {
    let a = [1,2,3,4,5];
    for element in a {
        println!("The element is {element}");
    }
}

fn rustier_countdown_loop() {
    for num in (1..4).rev() {
        println!("{num}...");
    }
    println!("Liftoff!")
}
