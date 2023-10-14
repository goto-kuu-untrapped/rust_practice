
fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n - 1),
    }

    /*
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
    */
}

fn main() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(4), 24);
}
