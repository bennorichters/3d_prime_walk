pub struct Primes {
    primes: Vec<u64>,
    index: usize,
}

impl Primes {
    pub fn with_limit(limit: usize) -> Self {
        let primes = Self::sieve_of_eratosthenes(limit);
        Primes { primes, index: 0 }
    }

    fn sieve_of_eratosthenes(limit: usize) -> Vec<u64> {
        if limit < 2 {
            return Vec::new();
        }

        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        let mut p = 2;
        while p * p <= limit {
            if is_prime[p] {
                for multiple in (p * p..=limit).step_by(p) {
                    is_prime[multiple] = false;
                }
            }
            p += 1;
        }

        is_prime
            .iter()
            .enumerate()
            .filter_map(|(num, &prime)| if prime { Some(num as u64) } else { None })
            .collect()
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.primes.len() {
            let prime = self.primes[self.index];
            self.index += 1;
            Some(prime)
        } else {
            None
        }
    }
}
