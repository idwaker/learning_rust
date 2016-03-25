
extern crate rand;

// importing
use std::io;
use rand::Rng;


// empty return type defaults to ()
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("The secret number is {}", secret_number);

    println!("Please input your guess:");

    // let creates a variable binding
    // we create a mutable variable binding
    // :: associated function ( somewhat like static function )
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}
