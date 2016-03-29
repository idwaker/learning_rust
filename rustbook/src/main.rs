
// however
use std::cell::Cell;


// mutability of struct is in its binding
struct Color {
    r: i32,
    g: i32,
    b: i32,
    // mut a: i32, // not possible
    // how ever
    a: Cell<f32>,
}


fn main() {
    let mut x = 5;
    let mut xx = 55;
    let mut z = 9;
    let mut zz = 99;
    
    // immutable binding to mutable reference
    // can change value of x but 
    // y cannot bind to another reference
    let y = &mut x;
    
    // mutable binding to mutable reference
    let mut yy = &mut xx;
 
    // cannot do this   
    // y = &mut z;
    
    // can do this
    yy = &mut zz;
    
    let c = Color {r: 32, g: 56, b: 89, a: Cell::new(1.0)};
    
    // cannot do
    // c.r = 23;
    // however
    c.a.set(0.5);
}
