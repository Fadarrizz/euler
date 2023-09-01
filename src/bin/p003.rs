fn main() {
    println!("{:?}", solve(600_851_475_143));
}

fn solve(max: u64) -> u32 {
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

        prime = euler::next_prime(prime as u32) as u64;
    }
    
    highest_prime_factor as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_factor() {
        assert_eq!(29, solve(13195));
    }
}
