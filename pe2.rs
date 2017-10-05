struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let next = self.curr + self.next;

        self.curr = self.next;
        self.next = next;

        Some(self.curr)
    }
}

fn main() {
    let fib = Fibonacci { curr: 1, next: 1 };
    let mut sum = 0;

    for i in fib {
        if i > 4000000 {
            break;
        }
        if i % 2 == 0 {
            sum += i;
        }
    }

    println!("{}", sum);
}
