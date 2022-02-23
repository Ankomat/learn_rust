fn main() {
    let mut s = String::from("hello");

    println!("{}", &mut s);
    println!("{}", s);


    change(&s);
}

fn change(some_string: &String) {
    println!("{}", some_string);
}