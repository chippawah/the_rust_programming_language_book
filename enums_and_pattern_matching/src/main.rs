fn main() {
    non_idiomatic_enums();
    idiomatic_enums();
    enums_with_multiple_types();
    fun_with_options();
    fun_with_matching();
}

// To declare different variants of the IPAddressKind enum we use ENUM_NAME::VARIANT_NAME

fn non_idiomatic_enums() {
    println!("\n---non_idomatic_enums---");
    #[derive(Debug)]
    enum IPAddressKind {
        V4,
        V6,
    }
    #[derive(Debug)]
    struct IPAddress {
        _kind: IPAddressKind,
        _address: String,
    }
    let home = IPAddress {
        _kind: IPAddressKind::V4,
        _address: String::from("127.0.0.1"),
    };
    let loopback = IPAddress {
        _kind: IPAddressKind::V6,
        _address: String::from("::1"),
    };
    println!(
        "This is only using half the potential of enums: home: {:?} loopback: {:?}",
        home, loopback
    )
}

fn idiomatic_enums() {
    println!("\n---idomatic_enums---");
    #[derive(Debug)]
    enum IPAddress {
        V4(u8, u8, u8, u8), // Enums also allow for different data types in each variant
        V6(String),
    }
    let home = IPAddress::V4(127, 0, 0, 1);
    let loopback = IPAddress::V6(String::from("::1"));
    println!(
        "This really uses enums more concisely home: {:?} loopback: {:?}",
        home, loopback
    );
}

fn enums_with_multiple_types() {
    println!("\n---enums_with_multiple_types---");
    #[derive(Debug)]
    // An enum with a wide variety of data types within its variants
    enum Message {
        Quit,                       // No data associated with it
        Move { _x: i32, _y: i32 },  // named fields like a struct
        Write(String),              // A string like the IPAddress::V6
        ChangeColor(i32, i32, i32), // Three unnamed numbers
    }
    // Enums can have methods defined on them as well:
    impl Message {
        fn call(&self) {
            let msg_type = match self {
                Message::Quit => "quit",
                Message::ChangeColor(_r, _g, _b) => "change_color",
                Message::Move { _x, _y } => "move",
                Message::Write(_msg) => "write",
            };
            println!("Called message type: {msg_type}");
        }
    }
    let message = Message::Quit;
    println!("Message: {:?}", message);
    message.call();
    let message = Message::Move { _x: 1, _y: 5 };
    println!("Message: {:?}", message);
    message.call();
    let message = Message::ChangeColor(0, 0, 0);
    println!("Message: {:?}", message);
    message.call();
    let message = Message::Write(String::from("Hello, World!"));
    println!("Message: {:?}", message);
    message.call();
}

/* The Option<T> enum:
   The standard library provides an enum, Option<T>, is included in the prelude so you can use its variants without Option::
   Using the Option<T> enum over a null value means the compiler wont allow use of a value that could be something or nothing
   Option<T> must be converted to the concrete type T before the value can be used
   enum Option<T> {
       None,
       Some(T),
   }
*/
fn fun_with_options() {
    println!("\n---fun_with_options---");
    let x: Option<i32> = None; // Must annotate the type when using None
    print_an_optional_i32(x);
    let y = Some(9); // Type can be infered by the compiler here
    catch_all_print_an_optional_i32(y);
    let z = Some(0);
    if_let_print_an_optional_i32(z);
}

// The match keyword can be used to handle Options like any other enum and it's variants:
// Because the compiler forces handling all variants or explicitly ignoring them we guarantee that the None case has been accounted for
fn print_an_optional_i32(val: Option<i32>) {
    match val {
        Some(num) => println!("Got {num} from the val argument!"),
        None => println!("The val argument was None!"),
    };
}
// Here the _ catch all pattern is used in combination with the unit value to indicate nothing should be done if the value doesn't match the Some(i32) pattern
fn catch_all_print_an_optional_i32(val: Option<i32>) {
    match val {
        Some(num) => println!("Got {num} from the val argument!"),
        _ => println!("The val argument didn't match the Some(i32) pattern!"),
    }
}
// Here if let is used to compare a pattern to the value and execute code only if that pattern is matched
// You can also include an else block with an if let statement
fn if_let_print_an_optional_i32(val: Option<i32>) {
    if let Some(num) = val {
        println!("Got {num} from the val argument!");
    } else {
        println!("The val argument didn't match the Some(i32) pattern! ...but with \"if let\" instead of match")
    }
}

#[derive(Debug)]
enum USState {
    // AL,
    // AK,
    AZ,
    // ...
}

enum USCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn fun_with_matching() {
    println!("\n---fun_with_matching---");
    let penny = USCoin::Penny;
    let val_in_cents = us_coin_value(penny);
    println!("The value of a penny is: {val_in_cents}/100 of a dollar");

    let nickel = USCoin::Nickel;
    let val_in_cents = us_coin_value(nickel);
    println!("The value of a nickel is: {val_in_cents}/100 of a dollar");

    let dime = USCoin::Dime;
    let val_in_cents = us_coin_value(dime);
    println!("The value of a dime is: {val_in_cents}/100 of a dollar");

    let quarter = USCoin::Quarter(USState::AZ);
    let val_in_cents = us_coin_value(quarter);
    println!("The value of a quarter is: {val_in_cents}/100 of a dollar");
}

/*
    Following the size of the US coin sizes to demonstrate the restrictiveness and evaluation of the patterns in descending order
    The dime is the smallest and would drop into any of the subsequent holes so it goes first followed by the penny, nickel, and quarter due to their increasing size
    That is to say the match statement will execute the *first* arm that the value matches and no others and that it will compare in the order listed in code
    Also note that the expressions following a match keyword can anything while the expression after an if statement must evaluate to a bool
*/
fn us_coin_value(coin: USCoin) -> u8 {
    match coin {
        USCoin::Dime => 10,
        USCoin::Penny => 1,
        USCoin::Nickel => 5,
        USCoin::Quarter(state) => {
            println!("The state on this quarter is: {:?}", state);
            25
        }
    }
}
