use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Please input your temperature in °C:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a valid temperature !");
                break;
            }
        };
        let output = input * 9.0 / 5.0 + 32.0;
        println!("{}°C is equal to {}°F.", input, output);
    }
}