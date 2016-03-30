
struct Point {
    x: i32,
    y: i32,
}

enum OptionalTuple {
    Value(i32, i32, i32),
    Missing,
}


fn main() {
    // patterns
    
    let c = 'c';
    
    // match all kind
    match c {
        x => println!("{}", x),
    }
    
    // _ match any
    
    // multiple patterns
    match c {
        'a' | 'b' => println!("match a or b"),
        'c' => println!("match c"),
        _ => println!("match any thing else"),
    }
    
    let origin = Point { x: 0, y: 0 };
    
    // destructuring with given name
    match origin {
        Point { x: x1, y: y1 } => println!("x: {}, y: {}", x1, y1),
    }
    
    // or without name
    match origin {
        Point { x, y } => println!("x: {}, y: {}", x, y),
    }
    
    // ignore some thing
    match origin {
        Point { x: x, .. } => println!("x: {}", x),
    }
    
    // match a result type Result<T, E>
    // _ ignore what we dont need
    // match some_error {
    //     Ok(val) => println!("we got a value: {}", value),
    //     Err(_) => println!("got some error"),
    // }
    
    // .. disregard multiple values
    
    let x = OptionalTuple::Value(5, -2, 3);
    
    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"),
        OptionalTuple::Missing => println!("No such luck."),
    }
    
    // want t get refernece
    
    match c {
        ref c => println!("got reference to {}", c),
    }
    
    
    let mut d = 'd';
    
    // or got a mutable reference
    match d {
        ref mut c => println!("got mutable reference to {}", c),
    }
    
    // or we can match ranges
    match d {
        'a' ... 'm' => println!("somewhere between a and m"),
        'n' ... 'z' => println!("rest of the world"),
        _ => println!("still need exhaust pattern apparantly"),
    }
    
    
    // bound with @
    match d {
        e @ 'a' ... 'm' => println!("somewhere between a and m is {}", e),
        'n' ... 'z' => println!("rest of the world"),
        _ => println!("still need exhaust pattern apparantly"),
    }
    
    //bound to be something in or statement
    match d {
        e @ 'a' ... 'e' | e @ 'x' ... 'z' => println!("somewhere between a and e or x to z is {}", e),
        'n' ... 'z' => println!("rest of the world"),
        _ => println!("still need exhaust pattern apparantly"),
    }
    
    // match guards
    match d {
        'c' | 'd' if c == 'c' => println!("this is what when you are ccc"),
        _ => println!("not very interesting is it?"),
    }
}
