fn main() {
    let user1 = User{
        signin_count: 0,
        active: true,
        email: String::from("foo@bar.com"),
        username: String::from("foobar"),
    };
    let email = user1.email; // we can no longer user user1's email after its ownership is transferred to the email variable (String doesn't implement Copy trait)
    println!("The user struct's email field is: {email}");
    // here we know that we want the same values for active and signin_count so we use the fields from the first user
    // worth noting that we're invalidating the user1 variable because the instantiation of user2's email and signin_count are *moved* into user2 because we haven't cloned it
    let user2 = User{
        signin_count: user1.signin_count,
        active: user1.active,
        email: String::from("user2@foo.com"),
        username: String::from("taptar"),
    };
    // We can also use the .. styntax to just get the rest of the fields not declared
    // Like the above, this will invalidate user3 so we can no longer use it
    let _user3 = User{
        email: String::from("user3@foo.com"),
        username: String::from("batbaz"),
        ..user2 // this must come last and cannot have a trailing comma
    }; 
    let user4 = user_builder(String::from("foo"), String::from("bar"));
    let username = user4.username; // This also invalidates the usage of user4's username
    println!("The username for user4 is {username}");

    // Here we define insances of a Point and a Color that would otherwise be identical tuples without clear meaning
    let _black = Color(0,0,0);
    let _origin = Point(0,0,0);
    // Here we declare an instance of AlwaysEqual like a tuple struct but without values passed in
    let _x = AlwaysEqual();

    println!("\n--- area_calc_fun ---");
    area_calc_fun();
    println!("\n--- fun_with_debug ---");
    fun_with_debug();
    println!("\n--- method_fun ---");
    method_fun();

}
// Worth noting that the User struct is using an owned type for all it's fields
// If we used &str instead we would need to annotate the lifetimes which is discussed later in the book
struct User {
    signin_count: u64,
    active: bool,
    email: String,
    username: String,
}

// Not very rusty to do it this way but traits are introduced later
// Here we'll show how to shorthand declarations of fields in a struct
fn user_builder(username: String, email: String) -> User {
    return User { signin_count: 0, active: true, email, username };
}

// These are both tuple structs that are used to give special meaning to a tuple and when naming the fields would be redundant
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);
// Unit types are also allowed when you want to implement traits on a type that has no data
// could be useful for testing
struct AlwaysEqual();

fn variable_area_calc(x: usize, y: usize) -> usize {
    x * y
}

fn tuple_area_calc(dimensions: (usize, usize)) -> usize {
    return dimensions.0 * dimensions.1;
}
#[derive(Debug)] // This isn't required until the fun_with_debug function.
struct Rectangle {
    height: usize,
    width: usize,
}

fn stuct_area_calc(rect: &Rectangle) -> usize {
    rect.height * rect.width
}

fn area_calc_fun() {
    let height = 10;
    let width = 20;
    
    // unlike with the String type we don't need to worry about height and width being moved here
    // Because they are instead copied because usize implements the Copy trait
    let area = variable_area_calc(height, width);
    println!("The area as determined by variable_area_calc is: {area}");

    let tuple_tangle = (height, width);
    let area = tuple_area_calc(tuple_tangle);
    println!("The area of the tuple_tangle is: {area}");

    let struct_tangle = Rectangle{
        height,
        width,
    };
    let area = stuct_area_calc(&struct_tangle);
    println!("The area of the struct_tangle is: {area}")

    
}

fn fun_with_debug() {
    let height = 10;
    let width = 2;
    // here we create a PrintableRect that has the Debug trait on it using the derive attribute
    let print_tangle = Rectangle{
        height,
        width,
    };
    println!("The print_tangle is {:?}", print_tangle); // using :? instead to keep it on one line otherwise we could do :#?
    // Using the dbg! macro we can print more info like line number and filename.
    // dbg! takes ownership of the expression passed in and returns it back to the caller scope
    let rect = Rectangle{ height, width };
    println!("This is printed to stdout with {{:?}} syntax and can be used because of the #[derive(Debug)] annotation on the struct: {:?}", rect);
    println!("The next output will be to stderr using the dbg! macro which uses :#? for an easier debugging experience along with line number and filename");
    dbg!(&rect); // using a reference here so we can get the area after   
    let area = dbg!(stuct_area_calc(&rect));
    println!("The area of rect is: {area}");
}

// Putting the implementation here because that's where it's introduced in the book *after* introducing the Debug trait with the derive attribute
impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
    // This is a general associated function and doesn't take self as a parameter. It will use the :: syntax when called
    fn square(size: usize) -> Rectangle {
        Rectangle { height: size, width: size }
    }
}

fn method_fun() {
    let height = 10;
    let width = 5;
    let rect = Rectangle{ height, width };
    let area = dbg!(rect.area());
    println!("The area of rect as determined by Rectangle's method (a special associated function) is: {area}");

    let rect2 = Rectangle{ height: 2 * height, width: 3 * width };
    let rect3 = Rectangle{ height: 5 * height, width: 10 * width };

    println!("rect: {:?}, rect2: {:?}, rect3: {:?}", rect, rect2, rect3);
    
    let can_hold = rect.can_hold(&rect2);
    println!("rect can hold rect2: {can_hold}");
    let can_hold = rect2.can_hold(&rect3);
    println!("rect2 can hold rect3: {can_hold}");
    let can_hold = rect3.can_hold(&rect);
    println!("rect3 can hold rect: {can_hold}");
    let can_hold = rect3.can_hold(&rect2);
    println!("rect3 can hold rect2: {can_hold}");

    let square = dbg!(Rectangle::square(10));
    println!("The square created using the associated function \"square\" is: {:?}", square)
}