pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;
    let mut digits_count = 0;

    for c in cc_number.chars().rev() {
        if c == ' ' {
            continue;
        }

        let digit = match c.to_digit(10) {
            Some(d) => d,
            None => return false,
        };

        digits_count += 1;

        if double {
            let doubled = digit * 2;
            sum += if doubled > 9 { doubled - 9 } else { doubled };
        } else {
            sum += digit;
        }

        double = !double;
    }

    digits_count >= 2 && sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_too_short() {
        assert!(!luhn("0"));
        assert!(!luhn(" 7 "));
    }

    #[test]
    fn test_invalid_characters() {
        assert!(!luhn("1234a567"));
        assert!(!luhn("12-34"));
    }
}

fn main() {
    // просто пример запуска
    let cc = "4263 9826 4026 9299";
    println!("{} -> {}", cc, luhn(cc));
}