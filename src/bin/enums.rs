#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => (),
            Message::Move{x,y  } => println!("{}, {}", x, y),
            Message::Write(t) => println!("{:?}", t),
            Message::ChangeColor(x, y, z) => println!("{}, {}, {}", x, y, z),
        }
    }
}

fn main() {
    let m1 = Message::Write(String::from("hello"));
    let m2 = Message::ChangeColor(1,2,3);
    m1.call();
    m2.call();
}
