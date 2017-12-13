fn next_prime(primes: &Vec<u32>) -> u32 {
    let mut n = *primes.last().unwrap();
    let mut found: bool = false;

    while !found {
        n += 2;

        found = true;
        for i in primes {
            if n % i == 0 {
                found = false;
                break;
            } else if i * i > n {
                break;
            }
        }
    }

    n
}

fn main() {
    let mut primes: Vec<u32> = Vec::new();
    let mut count = 0;

    primes.push(2);
    primes.push(3);
    count += 2;

    while count < 10001 {
        let n = next_prime(&primes);
        primes.push(n);
        count += 1;
    }

    println!("{}", primes.last().unwrap());
}
