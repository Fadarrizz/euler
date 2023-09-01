fn main() {
    println!("{:?}", solve(4_000_000));
}

fn solve(max: u64) -> u64 {
    let mut sum = 0;
    let mut a = 0;
    let mut b = 1;

    while b <= max {
        if b % 2 == 0 {
            sum += b;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum_even_numbers() {
        assert_eq!(44, super::solve(89));
    }
}

