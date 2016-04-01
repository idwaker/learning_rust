
// generics
struct Point<T> {
    x: T,
    y: T,
}


impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}


fn main() {

}
