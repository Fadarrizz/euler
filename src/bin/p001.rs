fn main() {
    println!("{:?}", solve(1000));
}

fn solve(max: u32) -> u32 {
    (0..max).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_ten() {
        assert_eq!(23, super::solve(10));
    }
}
