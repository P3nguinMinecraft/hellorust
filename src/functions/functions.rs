pub fn main() {
    println!("{}", add(5, 10)); // calling the add function and printing the result
    println!("{}", add2(5, 10)); // calling the add2 function and printing
    something();
}

fn something() {
    println!("This is something");
}

fn add(x: i32, y: i32) -> i32 { // function that takes two i32 parameters and returns an i32
    x + y
}

fn add2(x: i16, y: i32) -> i32 { // overloading does not exist, must use different name
    return x as i32 + y // we can use type casting to convert x to i32, can also use return keyword
}
