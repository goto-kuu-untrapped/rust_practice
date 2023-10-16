extern crate rand;

use std::io;
use rand::Rng;

enum COMPARED {
    Smaller,
    SameToSecret,
    Bigger,
    SameToFormerInput,
    Invalid 
}

fn create_secret_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100)
}

fn get_input_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input: not a number")
}

fn compare_input_and_secret(input: i32, secret: i32, former_input: i32) -> COMPARED {
    if input < 0 || 100 < input {
        return COMPARED::Invalid;
    }

    if input == former_input {
        COMPARED::SameToFormerInput
    } else if input == secret {
        COMPARED::SameToSecret
    } else if input < secret {
        COMPARED::Smaller
    } else {
        COMPARED::Bigger
    } 
}

fn count_up(mut count: i32) -> i32 {
    count += 1;
    count
}


fn main() {
    println!("This is Guess Game.");
    println!("We can use the number from 1 to 100.");

    // create a secret number
    let secret = create_secret_number();
    // record guess count
    let mut guess_count = 0;
    // ask user to input number
    let mut input;
    // record former guess number
    let mut former_input = 0;
    // tell input number is bigger or smaller than secret number
    loop {
        input = get_input_number();
        match compare_input_and_secret(input, secret, former_input) {
            COMPARED::Bigger => {
                // change count, but not to change is input number is same to before
                println!("Your guess is bigger than secret.");
                guess_count = count_up(guess_count);
                former_input = input;
            },
            COMPARED::Smaller => {
                println!("Your guess is smaller than secret.");
                guess_count = count_up(guess_count);
                former_input = input;
            },
            COMPARED::SameToFormerInput => {
                println!("Your guess is same to before.");
                former_input = input;
            }
            COMPARED::SameToSecret => {
                guess_count = count_up(guess_count);
                println!("Your guess is right!");
                break;
            },
            COMPARED::Invalid => {
                println!("Please use the number from 1 to 100.");
            }
        }
        println!("Let's guess one more time.");
    }
    println!("Your count to guess my secret: {}", guess_count);
}
