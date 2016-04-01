

fn main() {
    let victor = vec![1, 3, 9];
    
    let mut vicky = vec![77; 5]; // repeat 77 5 times
    
    // use usize for index
    let i: usize = 1;
    println!("second victor is {}", victor[i]);
    
    // doensot work
    // let j: i32 = 1;
    // println!("second victor is {}", victor[j]);
    
    // outof bound access
    // victor[7];
    // will get panic
    
    // indirect access with get
    match victor.get(7) {
        Some(x) => println!("well got value {}", x),
        None => println!("Well found nothing there."),
    }
    
    // iterating
    
    // reference
    for i in &vicky {
        println!("reference to: {}", i);
    }
    
    // mutable reference
    for i in &mut vicky {
        println!("mutable reference to: {}", i);
    }
    
    // ownership
    for i in vicky {
        println!("ownership to: {}", i);
    }
}
