

//! expression returns a value while statement do not

fn main() {
    println!("2 + 3 = {}", sum(2, 3));
    println!("2 ** 2 = {}", square(2));
    
    // this is nice
    let c: fn(i32) -> i32 = cubeme;
    
    // let c = cubeme;
    
    println!("2 ** 3 = {}", c(2));
    
    // panic only after completing above task
    // divergent();
    
    // it can be used as any type
    let att = divergent();
}


// parameters must declare types
// return type is optional
fn sum(a: u32, b: u32) -> u32 {
    // last expression always returns
    // should not use ; which gives ()
    a + b
}


fn square(a: u32) -> u32 {
    // rust has return also for some of the cases
    // power operation needs importing from std lib
    return a * a;
}


fn divergent() -> ! {
    // this function doesnot returns
    // makes a crash
    panic!("I am having panic attack :-(");
}


fn cubeme(a: i32) -> i32 {
    a * a * a
}

