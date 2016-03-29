
struct Point {
    x: i32,
    y: i32,
}


// tuple structs
// field dont have names
struct Color(i32, i32, i32);

// can be useful in situations
struct Inches(f64);


// unit like struct
struct None;


fn main() {
    let mut origin = Point { x: 0, y: 0 };
    
    println!("Point Origin x: {}, y: {}", origin.x, origin.y);
    
    let point = Point { x: 56, .. origin };
    
    println!("Point x: {}, y: {}", point.x, point.y);
    
    let black = Color(0, 0, 0);
    
    let length_in_inches = Inches(10.2);
}
