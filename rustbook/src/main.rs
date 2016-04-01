
// generics
struct Point<T> {
    x: T,
    y: T,
}


impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}


impl<T: PartialEq> Point<T> {
    fn is_uniform(&self) -> bool {
        self.x == self.y
    }
}


struct Circle {
    radius: f64,
}


trait HasArea {
    fn area(&self) -> f64;
}


impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}


struct Square {
    length: f64,
    width: f64,
}


impl HasArea for Square {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}


fn main() {
    let c = Circle { radius: 1.2 };
    
    println!("{}", c.area());
    
    let s = Square { length: 1.2, width: 4.5 };
    print_area(s);
}


fn print_area<T: HasArea>(shape: T) {
    println!("{}", shape.area());
}


// multiple trait bound use + sign
use std::fmt::Debug;

fn foo<T: Clone + Debug>(x: T) {
    x.clone();
    println!("{:?}", x);
}

// where clause
fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    println!("{:?}", y);
}


// inheritence
trait Foo {
}


trait Bar : Foo {
}
