
fn takes_slice(slice: &str) {
    println!("got {}", slice);
}

fn main() {
    // two types of string
    // &str and String
    
    // &str is string slices
    // has fixed size and cannot be mutated
    // reference to the sequence of UTF8 bytes
    
    let greeting = "Hello there."; // greeting: &'static str
    println!("{}", greeting);
    
    let multi = "this is
    weird syntax with newline and spaces";
    println!("{}", multi);
    
    let multi2 = "this is too\
    weird syntax without newline and spaces";
    println!("{}", multi2);
    
    // String is heap allocated and growable
    let mut hello = "hello".to_string();
    hello.push_str(" world!");
    
    println!("{}", hello);
    
    takes_slice(&hello);
    
    // string indexing is not supported
    let hachiko = "忠犬ハチ公";
    
    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }
    
    print!("\n");
    
    for c in hachiko.chars() {
        print!("{}, ", c);
    }
    
    print!("\n");
    
    print!("2nd char {:?} \n", hachiko.chars().nth(1));
    
    let h = &greeting[0..2];
    println!("{}", h);
    
    // doesnot work wtf so much for everything UTF
    // let hh = &hachiko[0..2];
    // println!("{}", hh);
    
    let h1 = "hello".to_string();
    let h2 = "world";
    
    let h3 = h1 + h2;
    println!("concatenated {}", h3);
    
    let h1 = "hello".to_string();
    let h2 = "world".to_string();
    let h3 = h1 + &h2;
    println!("&String are coersed to &str {}", h3);
}
