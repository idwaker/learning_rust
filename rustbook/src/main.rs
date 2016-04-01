
struct HasDrop;


// Drop trait provides a way to run some code when value goes out scope
impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("dropping");
    }
}


fn main() {
    let x = HasDrop;
}
