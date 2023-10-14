use std::io;

fn get_input_num() -> u32 {
    println!("Please enter a number for sum:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input: not a number")
}

fn calculate_sum_multiples_of_3_or_5(num: u32) -> u32 {
    (1..=num)
        // &n: not borrow value, only read
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

fn main() {
    let num = get_input_num();
    let sum = calculate_sum_multiples_of_3_or_5(num);
    println!("Sum from multiples of 3 or 5 to {}: {}", num, sum);
}
