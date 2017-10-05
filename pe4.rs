fn is_palindrome(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let mut digits = Vec::new();

    let mut num = num;
    while num != 0 {
        digits.push(num % 10);
        num /= 10;
    }

    for i in 0..(digits.len() / 2) {
        if digits[i] != digits[digits.len() - i - 1] {
            return false
        }
    }

    true
}

fn main() {
    let mut max = 0;

    for i in 100..1000 {
        for j in i..1000 {
            let n = i * j;
            if is_palindrome(n) && n > max {
                max = n;
            }
        }
    }

    println!("{}", max);
}
