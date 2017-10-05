fn main() {
    let mut i;
    let mut sum = 0;

    i = 0;
    while i < 1000 {
        sum += i;
        i += 3;
    }

    i = 0;
    while i < 1000 {
        if i % 3 != 0 {
            sum += i;
        }
        i += 5;
    }

    println!("{}", sum);
}
