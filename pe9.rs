fn main() {
    let sum = 1000;

    for c in 2..sum {
        let cc = c * c;

        for b in 1..c {
            let bc = b + c;
            let bb = b * b;

            if bc > sum || bb > cc {
                break;
            }

            for a in 0..b {
                let abc = a + bc;
                let aabb = a * a + bb;

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
