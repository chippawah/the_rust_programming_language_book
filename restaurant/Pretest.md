Pretest: 
## Multiple Choice

1. **What is a crate in Rust?**
    - [ ] a. An error handling feature
    - [x] b. The fundamental unit of building and distributing code
    - [ ] c. A tool for creating modules
    - [ ] d. The Rust package manager

2. **What is the `mod` keyword used for in Rust?**
    - [ ] a. Declaring a new struct
    - [ ] b. Importing a crate
    - [x] c. Declaring a new module
    - [ ] d. Managing memory

3. **What is the `use` keyword used for in Rust?**
    - [ ] a. To declare that a crate will be used
    - [x] b. To import a module or item into scope
    - [ ] c. To declare that a function will be used
    - [ ] d. To manage memory

4. **What is the purpose of the `pub` keyword in Rust?**
    - [a] a. To make a function public
    - [ ] b. To publish a crate
    - [ ] c. To indicate the end of a block of code
    - [ ] d. To make a variable public

## Open Ended

1. **Describe what a package is in Rust.**

    A package in rust is a set of code that's related and can reference other pieces within that code without needing to import them.

2. **What is the difference between a binary crate and a library crate?**
    
    A binary crate is one that compiles to a binary that can be used on its own without anything else being required.
    A library crate on the other hand is a crate that is imported into other crates to provide functionality but not a standalone executable.

3. **Describe how to create a new module in Rust.**
    
    To create a new module in Rust you use the `mod` keyword.

4. **Explain how the `use` keyword works in Rust.**

    The `use` keyword imports the items specified so they can be used within the scope of the `use` statement

5. **What is the purpose of the `pub` keyword in Rust? How is it used?**

    The `pub` keyword marks a function, struct, or enum as publicly avaialble for use to other code. Meaning that it can be imported with the `use` keyword

6. **Explain what privacy boundaries are in Rust.**

    Unsure of this one

## Revision Questions

1. **What is the role of Enums in Rust and how are they used with the `match` keyword?**
    
    Enums are a way to express variants of a common type like us coins `enum Coin { Penny, Dime, /*...*/ }`.
    They can then be used in a match statment to execute code based on what variant of the enum is for example:
    ```rust
    // enum declaration
    enum Coin {
        // variants
        Penny,
        Nickel,
        Dime,
        Quarter
    }
    fn get_coin_value(coin: Coin) -> u8 {
        // Usage of match statement with an enum
        return match coin {
            Coin::Penny => 1,
            Coin:: Dime => 10,
            Coin::Nickel => 5,
            Coin::Quarter => 25,
        };
    }
    let penny = Coin::Penny;
    let coin_value = get_coin_value(penny);
    ``` 

2. **Explain how Rust's memory safety rules around ownership, borrowing and lifetimes work.**

    Rust's rules for memory safety are structured such that there's only one owner of a piece of memory at any given time.
    If borrowing is required Rust ensures there's only one mutable reference or multiple immutible refeerences but not both.
    When borrowing the lifetime of the reference can either infered by the compiler or when it can't must be annotated explicitly.
    This ensures that the memory ebing pointed to by the reference is guaranteed to be valid either for the lifetime annotated or by the life of the owner of that data.
