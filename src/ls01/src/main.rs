fn main() {
    let v = vec![2, 4, 3, 1, 5];

    let mut ans;

    // ans = return_largest1(&v);
    ans = v.iter().max().unwrap();

    println!("{}", ans);
}

fn return_largest1(v: &Vec<i32>) -> i32 {
    let mut max = i32::MIN;

    for x in v {
        if max < *x {
            max = *x
        }
    }
    max
}

