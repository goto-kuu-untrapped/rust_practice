use std::io;

fn get_input_num() -> u32 {
    println!("Please enter a number for sum:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num = input.trim(); 
    let num_input = match num.parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    num_input
}

fn get_input_mode() -> String {
    println!("Please enter mode. 1:calculate sum 2:calculate factorio");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn calc_sum(num: u32) -> u32 {
    (1..=num).sum()
}

fn calc_factorial(num: u32) -> u32 {
    let sum = match num {
        0 => 1,
        1 => 1,
        _ => num * calc_factorial(num - 1),
    };
    sum
}


fn main() {
    // get input num
    let mut num = get_input_num();
    // while num is not 0, tell user input again, or exit.
    while num == 0 {
        println!("not a collect number. input again, or input ctrl+z to exit.");
        num = get_input_num();
    };
    // get input mode
    let mut mode = get_input_mode();
    // while num is not 0, tell user input again, or exit.
    while mode != "1" && mode != "2" {
        println!("not a collect mode. input again, or input ctrl+z to exit.");
        mode = get_input_mode();
    };

    // "1" is String, use reference of dereference *mode 
    match &*mode {
        "1" => println!("{}", calc_sum(num)),
        "2" => println!("{}", calc_factorial(num)),
        _ => println!(""), // nothing process
    };
}
