fn main() {
    use primes::{Sieve, PrimeSet};

    let mut pset = Sieve::new();

    // heavy process. not measure time. u128MAX: 340282366920938463463374607431768211455 
    for (ix, n) in pset.iter().enumerate().take(u128::MAX as usize) {
        println!("Prime {}: {}", ix, n);
    }
}
