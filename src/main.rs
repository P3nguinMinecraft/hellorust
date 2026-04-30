/*
Basic Setup
https://doc.rust-lang.org/stable/book/ch01-01-installation.html
Install rust on your machine, cargo is bundled

`cargo build` to build the project, `cargo run` to run the project
*/
use std::io;
use 
rand::random_range;

mod guess_game; // imports the guess_game module
mod pack1;
mod pack2;
mod functions;
mod datatypes;
mod control;

fn main() {
    /*
    Hello World!
    println! is a macro (type of executable function) that prints to console
    */
    
    // println!("Hello, world!");

    /*
    Mutability and printing
    mut keyword defines the variable as mutable, meaning it can be changed after it is defined
    by default, variables are immutable.
    */
    
    // let x: i32  = 5;
    // let mut y: u8 = 6;

    // println!("The value of x is: {}", x);
    // println!("The value of y is: {y}");

    // println!("5 more than x is : {}", x + 5);
    // // x = 10; // this will cause an error because x is not mutable
    // y = 67; // these are fine because y is mutable
    // y += 5;
    // let ystr: String = y.to_string();
    // let concat: String = "5 more than y is : ".to_string() + &ystr; // concatenation using + operator, we need to use & to get a reference to ystr because + takes ownership of the left operand and borrows the right operand
    // println!("{concat}");
    
    /*
    Inputs
    uses std::io library
    */
    
    // println!("Input a string: ");
    
    // let mut str: String = String::new();

    // std::io::stdin() // "use std::io;" header to minimize, see below example
    //     .read_line(&mut str)
    //     .expect("Failed to read line");

    // println!("You entered: {}", str);

    /*
    Basic parsing
    */

    // println!("Input a number: ");
    // let mut input_str: String = String::new();
    
    // io::stdin() // you can state "use std::io;"" header to define io and then use io::stdin() instead of std::io::stdin()
    //     .read_line(&mut input_str)
    //     .expect("Failed to read line");

    // let pnum: u32 = input_str.trim().parse().expect("Please type a number!");
    // println!("Your input +5 is: {}", pnum + 5);

    /*
    Input validation
    uses loop
    */

    // let number: u32 = loop { // loops until a valid number is entered
    //     let mut input: String = String::new();
    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");
    //     if let Ok(num) = input.trim().parse() { // .parse() returns a Result, we check if it's Ok and get the number, break will return the number
    //         break num;
    //     } else {
    //         println!("That is not a number. Please enter a valid number: ");
    //     }
    // };

    // println!("You entered: {}", number);

    /*
    Random
    uses rand library v0.10.1 (see Cargo.toml dependencies)
    */
    
    // let number: u32 = random_range(1..=100); // inclusive
    // println!("The random number is: {}", number);

    /*
    Guessing game
    refer to guess_game.rs for the implementation, we just call the run function from the module
    */

    // guess_game::run();

    /*
    Importing packages
    /pack1 and /pack2
    */

    // pack1::say_hi::hi();
    // pack1::say_bye::bye();

    // pack2::one::one();
    // pack2::two::two();

    /*
    Variables
    */

    // datatypes::vars::shadowing();
    // datatypes::vars::constants();
    // datatypes::vars::scope();
    // datatypes::vars::types();
    // datatypes::vars::math();

    /*
    Data types
    */

    // datatypes::types::tuples();
    // datatypes::types::arrays();

    /*
    Functions
    */

    functions::functions::main();

    /*
    Expressions
    */
    
    functions::expressions::main();

    /*
    if else
    */

    // control::conditional::main();

    /*
    Loop
    */

    // control::loops::inf_loop();
    // control::loops::break_loop();
    // control::loops::while_loop();
    // control::loops::for_loop();

}

