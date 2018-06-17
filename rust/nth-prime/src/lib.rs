pub fn nth(n: u32) -> Option<u32> {
    let mut primes: Vec<u32> = vec![2,3,5,7,11,13,17,19,23];
    if n < 1 {
        None
    } else {
        let mut x = primes[primes.len() - 1];
        while n as usize > primes.len() {
            x += 1;
            let mut is_prime = true;
            for p in &primes[..primes.len() / 2] {
                if x % p == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes.push(x);
            }
        }
        Some(primes[n as usize - 1])
    }
}
