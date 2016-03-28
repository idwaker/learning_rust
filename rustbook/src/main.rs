//! these are also doc comments
//! but for crates, modules or functions
//! triple quotes looks nicer than these sad


/// these are doc comments
/// boring kind of things to do here
/// ok has support for markdown inside
fn main() {

    // these are comments    
    
    let x = 5;
    
    // no bloody paren nice
    if x == 5 {
        println!("yes x is 5");
    } else if x == 6 {
        println!("me too");
    } else {
        println!("never see me");
    }
    
    // another thing with let if
    // so if is an expression, right! 
    let y = if x == 5 { x + 1 } else { x - 1 };
    
    println!("and y is: {}", y);
}
