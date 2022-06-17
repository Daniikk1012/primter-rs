//! # Prime Number Iterator Library
//!
//! This crate provides an implementation of a prime number iterator based on
//! the sieve of Eratosthenes.
//!
//! # Get started
//!
//! To get started, read the documentation for struct [`Primes`]
//!
//! [`Primes`]: Primes

#[deny(missing_docs)]

/// Collection of generated prime numbers.
pub struct Primes {
    sieve: Vec<bool>,
    primes: Vec<usize>,
}

impl Primes {
    /// Constructs a new [`Primes`] that contains numbers generated up to 3
    /// (inclusive).
    ///
    /// [`Primes`]: Primes
    pub fn new() -> Self {
        Primes { sieve: vec![true, true, false, false], primes: vec![2, 3] }
    }

    /// Generates primes up to at least `n`.
    ///
    /// # Example
    ///
    /// ```
    /// use primter::Primes;
    ///
    /// fn main() {
    ///     let mut primes = Primes::new();
    ///     primes.generate_to(100);
    ///
    ///     for n in 1..=100 {
    ///         if primes.is_prime(n) {
    ///             println!("{} is prime", n);
    ///         } else {
    ///             println!("{} is not prime", n);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn generate_to(&mut self, n: usize) {
        if self.sieve.len() > n {
            return;
        }

        let old_len = self.sieve.len();

        {
            let len = (n + 1).next_power_of_two();
            self.sieve.reserve(len - self.sieve.len());

            while self.sieve.len() < len {
                self.sieve.push(false);
            }
        }

        for prime in &self.primes {
            for number in ((old_len + prime - 1) / prime * prime
                ..self.sieve.len())
                .step_by(*prime)
            {
                self.sieve[number] = true;
            }
        }

        for prime in old_len..self.sieve.len() {
            if !self.sieve[prime] {
                for number in (prime * prime..self.sieve.len()).step_by(prime) {
                    self.sieve[number] = true;
                }

                self.primes.push(prime);
            }
        }
    }

    /// Generates primes so that the total amount is at least `amount`.
    ///
    /// # Example
    ///
    /// ```
    /// use primter::Primes;
    ///
    /// fn main() {
    ///     let mut primes = Primes::new();
    ///     primes.generate_amount(100);
    ///
    ///     for prime in primes.into_iter().take(100) {
    ///         println!("{}", prime);
    ///     }
    /// }
    /// ```
    pub fn generate_amount(&mut self, amount: usize) {
        while self.primes.len() <= amount {
            self.generate_to(self.sieve.len());
        }
    }

    /// Checks whether a number is prime.
    ///
    /// This method works faster the more primes are generated.
    pub fn is_prime(&self, n: usize) -> bool {
        if self.sieve.len() > n {
            !self.sieve[n]
        } else if n % 2 == 0 || n % 3 == 0 {
            false
        } else {
            for prime in &self.primes {
                if prime * prime > n {
                    return true;
                }

                if n % prime == 0 {
                    return false;
                }
            }

            let start = {
                let last = *self.primes.last().unwrap_or(&0);

                if last % 6 == 1 {
                    last - 1
                } else if last % 6 == 5 {
                    last + 1
                } else {
                    6
                }
            };

            for number in (start..).step_by(6) {
                if (number - 1) * (number - 1) > n {
                    break;
                }

                if n % (number - 1) == 0 || n % (number + 1) == 0 {
                    return false;
                }
            }

            true
        }
    }

    /// Returns an immutable reference to the underlying sieve of Eratosthenes.
    ///
    /// To check if number is in the sieve, simply use it as the index.
    ///
    /// # Example
    ///
    /// ```
    /// use primter::Primes;
    ///
    /// fn main() {
    ///     let mut primes = Primes::new();
    ///     primes.generate_to(10);
    ///     assert!(primes.sieve()[10]); // 10 is not prime
    /// }
    /// ```
    pub fn sieve(&self) -> &[bool] {
        &self.sieve
    }

    /// Returns an immutable reference to the underlying [`Vec`] of generated
    /// primes.
    ///
    /// [`Vec`]: Vec
    pub fn primes(&self) -> &[usize] {
        &self.primes
    }

    /// Constructs a borrowed iterator
    pub fn iter(&mut self) -> Iter {
        self.into_iter()
    }
}

impl IntoIterator for Primes {
    type Item = usize;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { primes: self, index: 0 }
    }
}

impl<'a> IntoIterator for &'a mut Primes {
    type Item = usize;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { primes: self, index: 0 }
    }
}

/// Owned iterator for [`Primes`].
///
/// [`Primes`]: Primes
pub struct IntoIter {
    primes: Primes,
    index: usize,
}

impl Iterator for IntoIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.primes.generate_amount(self.index);

        Some(self.primes.primes()[self.index - 1])
    }
}

/// Borrowed iterator for [`Primes`].
///
/// [`Primes`]: Primes
pub struct Iter<'a> {
    primes: &'a mut Primes,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.primes.generate_amount(self.index);

        Some(self.primes.primes()[self.index - 1])
    }
}

#[cfg(test)]
mod tests {
    use crate::Primes;

    #[test]
    fn empty() {
        assert_eq!(Primes::new().sieve, [true, true, false, false]);
    }

    #[test]
    fn len() {
        let mut primes = Primes::new();
        primes.generate_to(4);
        assert_eq!(primes.sieve().len(), 8);
    }

    #[test]
    fn primes() {
        let mut primes = Primes::new();
        primes.generate_to(10);

        assert_eq!(primes.primes(), [2, 3, 5, 7, 11, 13]);

        assert_eq!(
            primes.sieve(),
            [
                true, true, false, false, true, false, true, false, true, true,
                true, false, true, false, true, true
            ]
        );

        primes.generate_to(20);

        assert_eq!(primes.primes(), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);

        assert_eq!(
            primes.sieve(),
            [
                true, true, false, false, true, false, true, false, true, true,
                true, false, true, false, true, true, true, false, true, false,
                true, true, true, false, true, true, true, true, true, false,
                true, false
            ]
        );
    }

    #[test]
    fn is_prime() {
        let mut primes = Primes::new();

        assert_eq!(primes.is_prime(0), false);
        assert_eq!(primes.is_prime(1), false);
        assert_eq!(primes.is_prime(2), true);
        assert_eq!(primes.is_prime(3), true);
        assert_eq!(primes.is_prime(4), false);
        assert_eq!(primes.is_prime(5), true);
        assert_eq!(primes.is_prime(100), false);
        assert_eq!(primes.is_prime(101), true);

        primes.generate_to(101);

        assert_eq!(primes.is_prime(0), false);
        assert_eq!(primes.is_prime(1), false);
        assert_eq!(primes.is_prime(2), true);
        assert_eq!(primes.is_prime(3), true);
        assert_eq!(primes.is_prime(4), false);
        assert_eq!(primes.is_prime(5), true);
        assert_eq!(primes.is_prime(100), false);
        assert_eq!(primes.is_prime(101), true);
    }

    #[test]
    fn into_iter() {
        let mut result = Vec::new();

        for prime in Primes::new().into_iter().take(10) {
            result.push(prime);
        }

        assert_eq!(result, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn iter() {
        let mut result = Vec::new();

        for prime in Primes::new().iter().take(10) {
            result.push(prime);
        }

        assert_eq!(result, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
}
