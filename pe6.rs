fn main() {
    let n = 100;
    let mut sum_of_sq = 0;
    let mut sq_of_sum = 0;

    for i in 1..(n + 1) {
        sum_of_sq += i*i;
        sq_of_sum += i;
    }
    sq_of_sum *= sq_of_sum;

    println!("{}", sq_of_sum - sum_of_sq);
}
