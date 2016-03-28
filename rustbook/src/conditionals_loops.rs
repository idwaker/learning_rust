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
    
    // loop infinitely
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        
        if counter > 5 {
            // break out of loop
            break;
        }
    }
    
    // while
    let mut done = false;
    while !done {
        counter += 1;
        println!("Counter inside while: {}", counter);
        
        if counter > 10 {
            done = true;
        }
    }
    
    // for works with iterator
    // 1..10 is range ; end is exclusive
    for i in 1..10 {
        // then there is continue
        if i % 2 == 0 {
            continue;
        }
        println!("value of odd i: {}", i);
    }
    
    for (i, j) in (5..9).enumerate() {
        println!("Enumerate Index: {}, Value: {}", i, j);
    }
    
    // labelled loops
    'outer: for i in 1..5 {
        'inner: for j in 1..2 {
            if i == 2 { continue 'outer; }
            if i == 4 { continue 'inner; }
            println!("{}, {}", i, j);
        }
    }
}
