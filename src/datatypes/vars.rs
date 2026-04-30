pub fn shadowing() {
    let x = 5;

    let x = x + 1; // x is replaced by the new variable of the same name
    
    println!("The value of x is: {x}"); // 6

    let spaces = "   "; // this is a string type &str
    let spaces = spaces.len(); // this is a number type usize
    println!("The number of spaces is: {spaces}");


    let mut spaces2 = "   ";
    // spaces2 = spaces2.len(); // this will error because spaces2 is mut and .len() is a number not a string
}

pub fn constants() {
    const THIS_IS_A_CONSTANT: u32 = 67;
}

pub fn scope() {
    let x = 6;

    { // new scope, x on line 4 is not replaced by this x on line 7
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6
}

pub fn types() {
    /*
        Length	Signed	Unsigned
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64 	u64
        128-bit	i128	u128
Arch Dependent	isize	usize
    */
    let x = 2.0; // f64 floating point 
    let y: f32 = 3.0; // f32

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

pub fn math() {
    
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
