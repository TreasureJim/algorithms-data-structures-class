fn main() {
    let n = 1_000_001;

    // let mut n = String::new();
    // let n = std::io::stdin().read_line(&mut n).unwrap();

    let mut not_prime = vec![false; n/2];

    for i in (3..=(n as f64).sqrt() as usize).step_by(2) {
        if !not_prime[i/2] {
            for j in (i*i..n).step_by(i*2) {
                not_prime[j/2] = true;
            }
        }
    }

    let mut accu = 2;
    for i in (3..n).step_by(2) {
        if !not_prime[i/2] {
            accu += i;
        }
    }

    println!("The total of all {n}th primes is {accu}");
}
