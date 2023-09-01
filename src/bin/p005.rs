use std::collections::HashMap;

fn main() {
    println!("{:?}", solve(20));
}

fn solve(n: u32) -> u32 {
    let factors = (2..n).flat_map(euler::prime_factors);
    
    let mut map = HashMap::new();

    for (prime, exponent) in factors {
        map.entry(prime)
            .and_modify(|e: &mut u32| *e = (*e).max(exponent))
            .or_insert(exponent);
    }

    map.into_iter().fold(1, |acc, (p, e)| acc * p.pow(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evenly_divisible_by_up_to_ten() {
        assert_eq!(2520, solve(10));
    }
}
