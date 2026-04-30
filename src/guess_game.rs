use std::io;
use rand::random_range;
use std::cmp::Ordering;

pub fn run() { // public function
    let number: u32 = rand_number();
    println!("Guess the number between 1 and 100!");

    loop {
        println!("Input guess: ");
        let mut guess: String = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // match is like a switch statement, if the parse returns Ok, we get the number, if it returns Err, we print the error and continue the loop
            Ok(num) => num,
            Err(err) => {
                println!("Invalid input!");
                println!("Error: {}", err);
                continue;
            }
        };
        
        if print_comp(guess, number) {
            break;
        }
    }
}

fn rand_number() -> u32 {
    random_range(1..=100)
}

fn print_comp(guess: u32, number: u32) -> bool {
    match guess.cmp(&number) { // cmp returns an Ordering, we compare guess and number
        Ordering::Less => {
            println!("Too low!");
            false
        },
        Ordering::Greater => {
            println!("Too high!");
            false
        },
        Ordering::Equal => {
            println!("You win!");
            println!("The number was: {}", number);
            true
        }
    }

        // same as above match statement, but using if else
        /*
        if guess < number {
            println!("Too low!");
            return false; // or just false
        } else if guess > number {
            println!("Too high!");
            return false;
        } else {
            println!("You win!");
            println!("The number was: {}", number);
            return true;
        }
        */
}
