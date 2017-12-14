fn main() {
    let sum = 1000;

    for c in 2..sum {
        let cc = c * c;

        for b in 1..c {
            for a in 0..b {
                let abc = a + b + c;
                let aabb = a * a + b * b;

                if abc > sum || aabb > cc {
                    break;
                }

                if abc == sum && aabb == cc {
                    println!("{}", a * b * c);
                    return;
                }
            }
        }
    }
}
