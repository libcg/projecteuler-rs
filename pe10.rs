fn next_prime(primes: &Vec<u32>) -> u32 {
    let mut n = *primes.last().unwrap();

    'next: loop {
        n += 2;

        for i in primes {
            if n % i == 0 {
                break;
            } else if i * i > n {
                break 'next;
            }
        }
    }

    n
}

fn main() {
    let mut primes: Vec<u32> = Vec::new();
    let mut sum: u64 = 0;

    primes.push(2);
    primes.push(3);
    sum += 5;

    loop {
        let n = next_prime(&primes);

        if n >= 2000000 {
            break;
        }

        primes.push(n);
        sum += n as u64;
    }

    println!("{}", sum);
}
