use std::io;

fn ask_user_name() {
    println!("Hello! What's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).ok();

    match name.trim() {
        "Alice" => println!("Hello Alice!"),
        "Bob" => println!("Hello Bob!"),
        _ => println!("Sorry, I can return greet only user name is Alice or Bob."),
    };
}

fn main() {
    ask_user_name();
}
