/*
This sucks, but I am glad I wrote it
 */

pub struct PrimeSieveData {
    primes: Vec<usize>,
    circle: usize,
    new_circle: usize,
    primes_in_circle: usize,
    max_number_calculated: usize,
}

impl PrimeSieveData {
    pub fn new() -> PrimeSieveData {
        return PrimeSieveData {
            primes: vec![2, 3, 5, 7],
            circle: 1,
            new_circle: 1,
            primes_in_circle: 0,
            max_number_calculated: 7
        }
    }

    pub fn get_nth_prime(&mut self, n: usize) -> usize {
        while n > self.primes.len() {
            self.next();
        }
        return self.primes[n - 1]
    }

    pub fn check_is_prime(&mut self, n: usize) -> bool {
        self.calculate_up_to(n);
        self.binary_search(n, 0, self.primes.len())
    }

    pub fn calculate_up_to(&mut self, n: usize) {
        while n > self.max_number_calculated {
            self.next();
        }
    }

    fn next(&mut self) -> usize {
        let mut i = self.next_circle();
        self.max_number_calculated = i;
        while !self.eratosthenes(i) {
            i = self.next_circle();
            self.max_number_calculated = i;
        }
        self.primes.append(&mut vec![i]);
        i
    }

    fn binary_search(&self, n: usize, i: usize, j: usize) -> bool {
        let s = ((j - i) / 2) + i;

        if n == self.primes[s] {
            true
        } else if s == i {
            false
        } else if n < self.primes[s] {
            self.binary_search(n, i, s)
        } else {
            self.binary_search(n, s, j)
        }
    }

    fn eratosthenes(&self, i: usize) -> bool {
        let f = i as f64;
        let sqrt = f.sqrt();

        if sqrt % 1.0 == 0.0 {
            return false
        }

        for n in &self.primes {
            if *n as f64 > sqrt {
                break;
            }

            if i % n == 0 {
                return false
            }
        }

        return true
    }

    fn next_circle(&mut self) -> usize {
        self.update_circle();

        let mut i = self.max_number_calculated + 1;
        'circle_loop: loop {
            let modulo = i % self.circle;
            if modulo == 1 { break }

            let mut j = self.primes_in_circle - 1;
            while j < self.primes.len() {
                if modulo % self.primes[j] == 0 { break 'circle_loop }
                if self.primes[j] > modulo { break }
                j += 1
            }

            i += 1;
        }

        return i;
    }

    fn update_circle(&mut self) {
        let largest_prime = *self.primes.last().unwrap_or(&7_usize);

        while self.new_circle < largest_prime {
            self.circle = self.new_circle;
            self.new_circle *= self.primes[self.primes_in_circle];
            self.primes_in_circle += 1;
        }
    }
}

#[test]
fn produces_first_primes() {
    let mut prime_sieve_data = PrimeSieveData::new();
    prime_sieve_data.calculate_up_to(1_000);

    assert_eq!(false, prime_sieve_data.check_is_prime(1));
    assert_eq!(true, prime_sieve_data.check_is_prime(2));
    assert_eq!(true, prime_sieve_data.check_is_prime(3));
    assert_eq!(false, prime_sieve_data.check_is_prime(4));
    assert_eq!(true, prime_sieve_data.check_is_prime(5));
    assert_eq!(false, prime_sieve_data.check_is_prime(6));
    assert_eq!(true, prime_sieve_data.check_is_prime(7));
    assert_eq!(false, prime_sieve_data.check_is_prime(8));
    assert_eq!(false, prime_sieve_data.check_is_prime(9));
    assert_eq!(false, prime_sieve_data.check_is_prime(10));
    assert_eq!(true, prime_sieve_data.check_is_prime(11));
    assert_eq!(false, prime_sieve_data.check_is_prime(12));
    assert_eq!(true, prime_sieve_data.check_is_prime(13));

    assert_eq!(2, prime_sieve_data.get_nth_prime(1));
    assert_eq!(3, prime_sieve_data.get_nth_prime(2));
    assert_eq!(5, prime_sieve_data.get_nth_prime(3));
    assert_eq!(7, prime_sieve_data.get_nth_prime(4));
    assert_eq!(11, prime_sieve_data.get_nth_prime(5));
    assert_eq!(13, prime_sieve_data.get_nth_prime(6));
    assert_eq!(17, prime_sieve_data.get_nth_prime(7));
    assert_eq!(19, prime_sieve_data.get_nth_prime(8));
    assert_eq!(23, prime_sieve_data.get_nth_prime(9));
    assert_eq!(29, prime_sieve_data.get_nth_prime(10));
}
