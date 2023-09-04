fn main() {
    println!("{:?}", solve(2_000_000));
}

fn solve(n: u32) -> u64 {
    let mut prime_sum = 0;
    let mut curr_prime = 2;
    while curr_prime < n {
        prime_sum += curr_prime as u64;
        curr_prime = euler::next_prime(curr_prime);
    }

    prime_sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_summation_of_primes() {
        assert_eq!(17, super::solve(10));
    }
}
