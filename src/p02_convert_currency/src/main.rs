use rust_decimal::prelude::*;

// convert jpn to usd
fn jpn_to_usd(amount: Decimal) -> Decimal {
    let rate = Decimal::new(14971, 2); 
    
    amount * rate
}

fn usd_to_jpn(amount: Decimal) -> Decimal {
    let rate = Decimal::new(14971, 2); 
    
    amount / rate
}


fn main() {
    let price = Decimal::new(25, 0); 
    println!("{} yen is {} dollers.", price, jpn_to_usd(price));

    let price = Decimal::new(300, 0); 
    println!("{} dollers is {} yen.", price, usd_to_jpn(price));
}
