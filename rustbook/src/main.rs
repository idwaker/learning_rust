

fn main() {
    
    // v is owner of the vector object
    let v1 = vec![2, 4, 5];
    
    // this is not reference, but a move semantics
    let v2 = v1; // now on v1 is not allowed to use unless...
    
    // illegal
    // println!("{}", v1[0]);
    
    // now we pass v2 to take function
    // which is moved as well
    let v3 = take(v2);
    
    // so this is illegal also
    // as v3 is new owner here
    // println!("{}", v2[0]);
    
    // there are copy types as well
    // implemented Copy traits
    let i1 = 1;
    
    // this is simply copied
    let i2 = i1;
    
    println!("i1 is {}", i1);
    
    // all primitives implement copy traits
    
    // borrowing ... references
    let mut ans = take2(&v3);
    println!("first item of vector is {}", ans);
    
    {
        // y is a reference
        let y = &mut ans;
        
        // *y is needed to access value ... dereference
        *y += 1;
    }
    println!("ans added by one {}", ans);
    
    // rules
    // any borrow should last a scope no greater than that of owner
    // one or more references but exactly one &mut reference
}


/// using references
/// immutable bindings
/// we cannot change v inside take2
fn take2(v: &Vec<i32>) -> i32 {
    // v.push(5);
    v[0]
}

/// using mutable reference
fn take3(v: &mut Vec<i32>) -> i32 {
    // update new one and return old one
    v.push(5);
    v[v.len() - 1]
}


fn take(v: Vec<i32>) -> Vec<i32> {
    // does something here
    
    // pass ownership back
    v
}
