
extern crate rand;

// importing
use std::io;
use std::cmp::Ordering;
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

    let guess: u32 = guess.trim().parse()
                        .expect("Please enter a number");

    println!("You guessed {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Wel-done!"),
    }
}
