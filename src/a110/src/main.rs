fn main() {

}

fn leap1() {
    let mut next_year = 2024;
    let mut count = 0;
    while count < 20 {
        if next_year % 4 == 0 {
            println!("{} is leap year.", next_year.to_string());
            count += 1;
        }
        next_year += 1;
    }
}

fn leap2() {
    let mut v: Vec<u32> = Vec::new();
    for year in 2024..2500 {
        if year % 4 == 0 {
            v.push(year); 
        }
        if v.len() == 20 {
            break;
        }
    }

    for year in &v {
        println!("{} is leap year.", year);
    }
}

fn leap3() {
    let leap_years: Vec<u32> = (2024..).step_by(4).take(20).collect();

    for year in &leap_years {
        println!("{} is leap year.", year);
    }
}
