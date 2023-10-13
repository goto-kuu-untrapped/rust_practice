use std::io;

fn ask_user_name() -> String {
    println!("Hello! What's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).ok();
    name.trim().to_string()
}

fn main() {
    let your_name = ask_user_name();
    println!("Hello {}!", your_name);
}
