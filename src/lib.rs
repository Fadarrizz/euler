pub type PrimeFactor<T> = (T, u32);

pub fn prime_factors(n: u32) -> Vec<PrimeFactor<u32>> {
    let mut factors = vec![];
    let mut prime = 2;
    let mut remaining = n;

    while remaining > 1 {
        if remaining % prime == 0 {
            let mut exponent = 1;
            remaining = remaining / prime;

            while remaining % prime == 0 {
                exponent += 1;
                remaining = remaining / prime;
            }

            factors.push((prime, exponent));
        }

        prime = next_prime(prime);
    }

    factors
}

pub fn next_prime(n: u32) -> u32 {
    let mut num = n + 1;
    while !is_prime(num) {
        num += 1;
    }
    num
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 || n == 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(0) == false);
        assert!(is_prime(1) == false);
        assert!(is_prime(2) == true);
        assert!(is_prime(3) == true);
        assert!(is_prime(4) == false);
        assert!(is_prime(5) == true);
        assert!(is_prime(6) == false);
        assert!(is_prime(7) == true);
        assert!(is_prime(11) == true);
        assert!(is_prime(13) == true);
        assert!(is_prime(28) == false);
        assert!(is_prime(29) == true);
    }

    #[test]
    fn test_next_prime() {
        assert_eq!(2, next_prime(0));
        assert_eq!(2, next_prime(1));
        assert_eq!(5, next_prime(2));
        assert_eq!(7, next_prime(5));
        assert_eq!(11, next_prime(8));
    }

    #[test]
    fn test_prime_factors() {
        fn check(n: u32, expected: &[PrimeFactor<u32>]) {
            assert_eq!(expected, &prime_factors(n)[..]);
        }

        check(0, &[]);
        check(1, &[]);
        check(2, &[(2, 1)]);
        check(3, &[(3, 1)]);
        check(4, &[(2, 2)]);
        check(5, &[(5, 1)]);
        check(6, &[(2, 1), (3, 1)]);
        check(7, &[(7, 1)]);
        check(8, &[(2, 3)]);
        check(9, &[(3, 2)]);
        check(10, &[(2, 1), (5, 1)]);
    }
}
