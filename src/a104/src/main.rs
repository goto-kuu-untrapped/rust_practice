use std::io;

fn get_input_num() -> u32 {
    println!("Please enter a number for sum:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input: not a number")
}

fn calculate_sum(num: u32) -> u32 {
    (1..=num).sum()
}

fn main() {
    let num = get_input_num();
    let sum = calculate_sum(num);
    println!("Sum from 1 to {}: {}", num, sum);
}
