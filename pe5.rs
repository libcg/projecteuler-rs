fn main() {
    let mut n: u32 = 0;

    loop {
        let mut found = true;

        n += 20;

        for i in 3..20 {
            if n % i != 0 {
                found = false;
                break;
            }
        }

        if found {
            break;
        }
    }

    println!("{}", n);
}
