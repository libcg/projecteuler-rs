use std::f64;

fn main() {
    let mut num: u64 = 600851475143;
    let mut i;
    let mut max = f64::sqrt(num as f64) as u64;

    i = 2;
    while i < max {
        if num % i == 0 {
            num /= i;
            max = f64::sqrt(num as f64) as u64;
        }

        i += 1;
    }

    println!("{}", num);
}
