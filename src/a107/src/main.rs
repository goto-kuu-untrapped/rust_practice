use std::iter;

fn main() {
    // print the title
    println!("*** Multiplication for numbers up to 12 ***");

    let upper = 12;
    // print the header row
    println!("| Multiple | {} |", 
              (1..=upper)
              .map(|n| format!("{:5}", n))
              .collect::<String>());

    // print the separator row
    println!("| -------- | {} |", iter::repeat("-").take(5*upper).collect::<String>());

    // Print the multiplication table
    for n in 1..=upper {
        let row: String = (1..=upper)
            .map(|m| format!("{:5}", n * m))
            .collect();

        println!("|  {:7} | {} |", n, row);
    }

}
