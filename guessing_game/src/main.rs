
// importing
use std::io;


// empty return type defaults to ()
fn main() {
    println!("Guess the number");

    println!("Please input your guess:");

    // let creates a variable binding
    // we create a mutable variable binding
    // :: associated function ( somewhat like static function )
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}
