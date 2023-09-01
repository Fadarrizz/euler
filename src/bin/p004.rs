fn main() {
    println!("{:?}", solve(3));
}

fn solve(digits: u8) -> u32 {
    let max = (10 as u32).pow(digits as u32);
    let min = (10 as u32).pow((digits - 1) as u32);
    let mut largest_palindrome = 0;

    for i in (min..max).rev() {
        for j in (min..=i).rev() {
            let product = i * j;
            if is_palindrome(product) && product > largest_palindrome {
                largest_palindrome = product;
            }
        }
    } 
    
    largest_palindrome
}

fn is_palindrome(n: u32) -> bool {
    let string = n.to_string();

    string.chars().rev().collect::<String>() == string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_two_digit_palindrome() {
        assert_eq!(9009, solve(2))
    }
}
