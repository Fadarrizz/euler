fn main() {
    println!("{:?}", solve(100));
}

fn solve(n: u64) -> u64 {
    let sum: u64 = (1..=n).map(|d| d.pow(2)).sum();
    let square: u64 = (1..=n).sum::<u64>().pow(2);

    square - sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum_square_difference_up_to_ten() {
        assert_eq!(2640, super::solve(10));
    }
}
