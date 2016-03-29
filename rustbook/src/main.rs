

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
    
    // lifetimes
    let yy = &5; // same as let _y = 5; let yy = &_y;
    let c = Color { x: yy };
    
    println!("Color {}", c.rgb());
    
    //has scope of program
    let hello: &'static str = "hello world!";
    
    // global scope
    static FOO: i32 = 5;
}


/// explicit lifetime
/// lifetime are one kind of generic parameters
/// life has one lifetime 'a
fn life<'a>(v: &'a Vec<i32>) {
    // the life of pi -- the lifetime of a
}


fn life2<'a, 'b>(x: &'a i32, y: &'b mut i32) {
    // this have two lifetimes
    // with y being mutable
    // mutable reference to i32 with lifetime 'b
}


// will need explicit lifetime if working with reference
// this declares lifetime
// any reference to Color cannot outlive reference to i32 it contain
struct Color<'a> {
    // and this uses it
    x: &'a i32,
}

// implementation
impl<'a> Color<'a> {
    fn rgb(&self) -> &'a i32 { self.x }
}


/// x and y both alive for same scope and return value is also alive for that scope
/// input lifetime -- associated with input/parameter of the function
/// output lifetime -- associated with output/return value of a function
fn halflife<'a>(x: &'a i32, y: &'a i32) -> &'a i32 { x }

/// x and y have different scopes but return value have same scope that of x
/// so i cannot return y from here since it has different lifetime ??
fn quaterlife<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { x }


/// using references
/// immutable bindings
/// we cannot change v inside take2
/// implicit lifetime
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


// example from book

// fn print(s: &str); // elided
// fn print<'a>(s: &'a str); // expanded

// fn debug(lvl: u32, s: &str); // elided
// fn debug<'a>(lvl: u32, s: &'a str); // expanded OK

// In the preceding example, `lvl` doesn’t need a lifetime because it’s not a
// reference (`&`). Only things relating to references (such as a `struct`
// which contains a reference) need lifetimes.

// fn substr(s: &str, until: u32) -> &str; // elided
// fn substr<'a>(s: &'a str, until: u32) -> &'a str; // expanded

// fn get_str() -> &str; // ILLEGAL, no inputs

// fn frob(s: &str, t: &str) -> &str; // ILLEGAL, two inputs
// fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // Expanded: Output lifetime is ambiguous

// fn get_mut(&mut self) -> &mut T; // elided
// fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded

// fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided
// fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

// fn new(buf: &mut [u8]) -> BufWriter; // elided
// fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded


