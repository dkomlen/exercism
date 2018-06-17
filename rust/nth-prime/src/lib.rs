pub fn nth(n: u32) -> Option<u32> {
    let mut primes = vec![2,3,5,7,11,13,17,19,23];
    match n {
        x if x < 1 => None,
        _ => {
            let mut x = primes[primes.len() - 1];
            while n as usize > primes.len() {
                x += 2;
                if primes.iter()
                    .take_while(|p| **p < (x as f32).sqrt() as u32 + 1)
                    .all(|p| x % p != 0) {
                        primes.push(x);
                }
            }
            Some(primes[n as usize -1])
        }
    }
}
