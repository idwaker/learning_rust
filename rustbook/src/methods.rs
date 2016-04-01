
struct Circle {
    x: i32,
    y: i32,
    r: f64,
}


impl Circle {

    // associated function
    fn new(x: i32, y: i32, radius: f64) -> Circle {
        Circle { x: x, y: y, r: radius }
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.r * self.r) 
    }
    
    // method chaining
    fn grow(&self, incr: f64) -> Circle {
        Circle { x: self.x, y: self.y, r: self.r + incr }
    }
}


// can write as many impl as possible
impl Circle {   
    // types of self
    fn ownership_over_self(self) {
        // do something
    }
    
    // borrowing
    fn reference_of_self(&self) {
        // prefer this over other two
    }
    
    fn mutable_reference_of_self(&mut self) {
        // do something
    }
}


// Builder pattern
struct CircleBuilder {
    x: i32,
    y: i32,
    r: f64,
}


impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0, y: 0, r: 0.0 } 
    }
    
    fn x(&mut self, x: i32) -> &mut CircleBuilder {
        self.x = x;
        self
    }
    
    fn y(&mut self, y: i32) -> &mut CircleBuilder {
        self.y = y;
        self
    }
    
    fn radius(&mut self, r: f64) -> &mut CircleBuilder {
        self.r = r;
        self
    }
    
    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, r: self.r }
    }
}


fn main() {
    let circle = Circle { x: 0, y: 0, r: 1.2 };
    
    println!("The area of circle is {}", circle.area());
    
    println!("The area of grown circle {}", circle.grow(0.5).area());
    
    let orange = Circle::new(5, 7, 2.7);
    println!("The area of orange circle is {}", orange.area());
    
    // using a builder
    let blue = CircleBuilder::new()
                .x(7)
                .y(9)
                .radius(1.9)
                .finalize();
    println!("The area of blue circle is {}", blue.area());
}

