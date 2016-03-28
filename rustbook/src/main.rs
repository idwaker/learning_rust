fn main() {

    // binding variable
    let hello = "Hello World";
    
    // left hand is pattern
    let (a, b) = (2, 4);
    
    let sum = a + b;
    
    // binding with mutable type, annotated
    let mut counter: u32 = 0;
    
    // we update the value
    counter += 1;
    
    println!("{}! and sum is: {}, and counter is: {}", hello, sum, counter);
    
    let x: u32 = 99876;
    {
        // this is the scope of y
        let y: u32 = 786;
        
        // this shadows the outer x for this block only
        let x = 987;
        
        println!("Inner, x: {}, y: {}", x, y);
        
    }
    
    // but this is same outer x
    println!("Outer, x: {}", x);
    
    // throws error because y is not in scope
    // println!("Outer, x: {}, y: {}", x, y);
    
    let mut m: i32 = 888;
    
    println!("Initial, m: {}", m);
    
    {
        // this should change value of m
        m = 999;
        println!("Inside a block, m: {}", m);
    }
    
    let m = m;
    println!("Now m becomes immutable with value: {}", m);
}
