use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use bigdecimal::RoundingMode;  

fn main() {
    let mut sum: BigDecimal = BigDecimal::from(0);

    for k in 1..10000 {
        sum += calculate_series(k);
    }
    println!("Sum of series is {sum}");
}

fn calculate_series(k: u32) -> BigDecimal {
    let _k = k as i64;

    let _child: i64 = -1 * &_k + 1;
    let child = BigDecimal::from(BigDecimal::from_i64(_child).unwrap());

    let _parent: i64 = 2 * &_k - 1;
    let parent = BigDecimal::from(BigDecimal::from_i64(_parent).unwrap());

    let calced = child / parent;
    calced.with_scale_round(5, RoundingMode::HalfEven)
}
