pub struct Primes {
    primes: Vec<u64>,
}

impl Primes {
    pub fn new() -> Self {
        Primes { primes: Vec::new() }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let candidate = match self.primes.last() {
            None => 2,
            Some(&2) => 3,
            Some(&p) => {
                let mut c = p + 2;
                while !self.is_prime(c) {
                    c += 2;
                }
                c
            }
        };
        self.primes.push(candidate);
        Some(candidate)
    }
}

impl Primes {
    fn is_prime(&self, n: u64) -> bool {
        for &p in &self.primes {
            if p * p > n {
                return true;
            }
            if n.is_multiple_of(p) {
                return false;
            }
        }
        true
    }
}
