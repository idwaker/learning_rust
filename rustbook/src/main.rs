

fn main() {
    
    // v is owner of the vector object
    let v1 = vec![2, 4, 5];
    
    // this is not reference, but a move semantics
    let v2 = v1; // now on v1 is not allowed to use unless...
    
    // illegal
    // println!("{}", v1[0]);
    
    // now we pass v2 to take function
    // which is moved as well
    take(v2);
    
    // so this is illegal also
    // println!("{}", v2[0]);
    
    // there are copy types as well
    // implemented Copy traits
    let i1 = 1;
    
    // this is simply copied
    let i2 = i1;
    
    println!("i1 is {}", i1);
    
    // all primitives implement copy traits
}


fn take(v: Vec<i32>) {
    // does something here
}
