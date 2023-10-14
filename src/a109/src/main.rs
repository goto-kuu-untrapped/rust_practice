enum COMPARED {
    BIGGER,
    SMALLER,
    SAME_TO_SECRET,
    SAME_TO_FORMER_INPUT
};

fn create_secret_number() -> u32 {

}

fn get_input_number() {

}

fn compare_input_and_secret(num_str String) -> COMPARED {

}

fn count_up(count: u32) -> u32 {
    count++
}


fn main() {
    println!("This is Guess Game.");

    // create a secret number
    let secret = create_secret_number();
    // record guess count
    let guess_count = 0;
    // ask user to input number
    let mut input;
    // tell input number is bigger or smaller than secret number
    loop {
        input = get_input_number();
        match compare_input_and_secret(input) {
            COMPARED::BIGGER => {
                // change count, but not to change is input number is same to before
                println!();
                count_up();
            },
            COMPARED::SMALLER => {
                println!();
                count_up();
            },
            COMPARED::SAME_TO_FORMER_INPUT => {\
                println!();
            }
            COMPARED::SAME_TO_SECRET => {
                println!();
                break;
            },
        }
    }
    println!("{}", guess_count);
}
