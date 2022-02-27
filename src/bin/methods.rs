#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn rotate(&mut self) {
        let temp = self.width;
        self.width = self.height;
        self.height = temp;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let mut r = Rectangle::new(50, 30);
    println!("The rectangle is {:?}", r);
    println!("The area of this rectangle is {}", r.area());
    r.rotate();
    println!("The rectangle is {:?}", r);
    println!("The area of this rectangle is {}", r.area());

    let r2  = Rectangle::new(20,30);

    if r.can_hold(&r2) {
        println!("OK");
    } else {
        println!("NOK")
    }
}