
// enums
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}


fn main() {
    let x: Message = Message::Move { x: 12, y: 19 };
    
    let m = Message::Write("hello world".to_string());
    
    let x = 3;
    
    match x {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("something else"),
    }
    
    // with let
    let number = match x {
        1 => "1",
        2 => "2",
        3 => "3",
        _ => "0"
    };
    
    println!("{}", number);
    
    match m {
        Message::Quit => println!("Quit"),
        Message::ChangeColor(r, g, b) => println!("Change Color"),
        Message::Move {x: x, y: y} => {
            // do something else here
            println!("Move");
        },
        Message::Write(x) => println!("{}", x),
    }
}
