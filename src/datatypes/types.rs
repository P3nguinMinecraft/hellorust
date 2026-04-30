
pub fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple of i32, f64, u8
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0; // access element through index
    let six_point_four = tup.1;
    let one = tup.2;
}

pub fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 5 elements of i32
    let b = [3; 5]; // 5 elements of 3 (implicit i32)

    let first = a[0];
    let second = a[1];
}