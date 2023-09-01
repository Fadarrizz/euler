use euler::next_prime;

fn main() {
    println!("{:?}", solve(10_001));
}

fn solve(n: u32) -> u32 {
    let mut prime = 2;
    for _ in 1..n {
        prime = next_prime(prime);
    }

    prime
}

#[cfg(test)]
mod tests {
    fn test_sixth_prime() {
        assert_eq!(13, super::solve(6));
    }
}
