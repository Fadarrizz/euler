fn main() {
    println!("{:?}", solve(600_851_475_143));
}

fn solve(max: u64) -> u64 {
    let mut highest_prime_factor = 2;   
    let mut prime = 2;
    let mut n = max;

    while n > 1 {
        if n % prime == 0 {
            highest_prime_factor = prime;

            while n % prime == 0 {
                n = n / prime;
            }
        }

        prime = next_prime(prime);
    }
    
    highest_prime_factor
}

fn next_prime(n: u64) -> u64 {
    let mut prime = n;
    let mut done = false;
    while done == false {
        prime += 1;
        if is_prime(prime) {
            done = true;
        }
    }

    return prime;
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 {
        return true;
    } else if n % 2 == 0  || n % 3 == 0 {
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
    fn prime_factor() {
        assert_eq!(29, solve(13195));
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(0) == false);
        assert!(is_prime(1) == false);
        assert!(is_prime(2) == true);
        assert!(is_prime(3) == false);
        assert!(is_prime(4) == false);
        assert!(is_prime(5) == true);
        assert!(is_prime(6) == false);
        assert!(is_prime(7) == true);
        assert!(is_prime(11) == true);
        assert!(is_prime(13) == true);
        assert!(is_prime(28) == false);
        assert!(is_prime(29) == true);
    }
}
