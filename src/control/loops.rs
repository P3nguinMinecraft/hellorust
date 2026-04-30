pub fn inf_loop() {
    loop {
        println!("This is an infinite loop!");
    }
}

pub fn break_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result); // 20
}

pub fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("done");
}


pub fn for_loop() {
    // we use _ to indicate that the variable is unused
    let _ = 1..5; // 1 inclusive to 5 exclusive
    let _ = 1..=5; // 1 inclusive to 5 inclusive
    let _ = (1..=5).rev(); // 5 inclusive to 1 inclusive in reverse
    
    for i in 1..=5 {
        println!("i: {}", i);
    }

    for j in (1..=5).rev() {
        println!("j: {}", j);
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("value: {}", element);
    }

    for number in (1..4).rev() { // range from 1 to 3, rev() reverses the order
        println!("{}!", number);
    }

    println!("done");
}
