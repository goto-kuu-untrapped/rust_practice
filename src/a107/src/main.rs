use std::iter;

fn main() {
    println!("*** Multiplication for numbers up to 12 ***");
    let upper = 12;
    print!("|multiple");
    for n in 1..=upper {
        print!("| {n:3} ");
    } 
    println!("|");

    for _ in 1..=20 {
        print!("====");
    } 
    println!("");

    for n in 1..=upper {
        print!("| {n:5} |");
        for m in 1..=upper {
            print!("| {:3} ", n * m);
        }
        println!("|");
    }

}
