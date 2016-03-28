

fn main() {
    // booleans
    let b = false;
    
    let a: bool = true;
    
    println!("bools a: {}, b: {}", a, b);
    
    // char is 4 byte long, so holds unicode
    let c: char = 'a';
    
    // chars are single quotes
    let d: char = 'न';
    
    println!("chars {} and {}", c, d);
    
    // numerics fixed sized
    
    let i = 32; // is i32
    let j = 1.0; // is f64
    
    let k: i8 = 12; // int 8 bit
    let l: u32 = 22; // unsigned int 32 bit
    
    println!("numerics i: {}, j: {}, k:{}, l: {}", i, j, k, l);
    
    // variable size
    let m: isize = 12; // depends on machine pointer size
    let n: usize = 13;
    
    println!("numerics vars m: {}, n: {}", m, n);
    
    // arrays
    let arr = [1, 2, 3]; // a: [i32; 3]
    let mut arr2 = ["John", "Jane", "Joseph", "Jade", "Jessica"]; // a: [&str; 2]
    
    // initialize array
    let arr3 = [0; 20]; // length 20 filled with 0
    
    println!("Lengths, arr1: {}, arr2: {}, arr3: {}", arr.len(), 
                                                      arr2.len(), arr3.len());
    println!("arr2 first item: {}", arr2[0]);
    
    // array slices
    let complete = &arr2[..]; // slice of all
    println!("original: {}, duplicate: {}", arr2.len(), complete.len());
    
    let partial = &arr2[1..4]; // [1] [2] and [3] : last one is exclusive
    println!("original: {}, partial: {}", arr2.len(), partial.len());
    
    // tuples
    let t = (1, "hello");
    
    // or
    let u: (i32, &str) = (2, "world");
    
    // (0,) well same as python
    
    println!("value of tuple at index 1: {} {}!", t.1, u.1);
    // remember tuple indexing is using dot operator unlike python
    
    // function are typed as well
    let x: fn(i32) -> i32;
}
