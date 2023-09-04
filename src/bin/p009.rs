fn main() {
    println!("{:?}", solve(1000));
}

fn solve(n: u32) -> u32 {
    for a in 1..n {
        for b in a..n-a {
            let c = n - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return a * b * c;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pythagorean_triplet() {
        assert_eq!(60, super::solve(12));
    }
}
