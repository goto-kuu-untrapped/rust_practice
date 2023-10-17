use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use bigdecimal::RoundingMode;  

fn main() {
    let mut sum = 0.0;
    for k in 1..=1_000_000 {
        let sign = (-1.0_f64).powi(k as i32 + 1);
        let denominator = 2.0 * k as f64 - 1.0;
        sum += sign / denominator;
    }
    let pi = sum * 4.0;
    println!("The value of pi is approximately: {}", pi);

}

/*
fn test() {
    let mut sum: BigDecimal = BigDecimal::from(0);

    for k in 1..100000 {
        sum += calculate_series(k);
    }
    println!("Sum of series is {}", sum * 4);
}

fn calculate_series(k: u32) -> BigDecimal {
    let _k = k as i64;

    let _child: f64 = (-1.0_f64).powi(k as i32 + 1);
    let child = BigDecimal::from(BigDecimal::from_f64(_child).unwrap());

    let _parent: i64 = 2 * &_k - 1;
    let parent = BigDecimal::from(BigDecimal::from_i64(_parent).unwrap());

    let calced = child / parent;
    calced.with_scale_round(3, RoundingMode::HalfEven)
}
*/
