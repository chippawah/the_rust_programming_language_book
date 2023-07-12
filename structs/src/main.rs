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


