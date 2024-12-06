use regex::Regex;
use std::cmp;

pub fn parse_instructions(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results: Vec<(i32, i32)> = Vec::new();
    for captures in re.captures_iter(input) {
        if let (Some(a), Some(b)) = (captures.get(1), captures.get(2)) {
            if let (Ok(a), Ok(b)) = (a.as_str().parse::<i32>(), b.as_str().parse::<i32>()) {
                results.push((a, b));
            }
        }
    }
    return results;
}

pub fn parse_instructions_and_sum(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for captures in re.captures_iter(input) {
        if let (Some(a), Some(b)) = (captures.get(1), captures.get(2)) {
            if let (Ok(a), Ok(b)) = (a.as_str().parse::<i32>(), b.as_str().parse::<i32>()) {
                total += a * b;
            }
        }
    }
    return total;
}

pub fn parse_instructions_no_regex(input: &str) -> Vec<(i32, i32)> {
    let mut results: Vec<(i32, i32)> = Vec::new();
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < input.len() - 4 {
        if bytes[i] == b'm' && bytes[i + 1] == b'u' && bytes[i + 2] == b'l' && bytes[i + 3] == b'(' {
            i += 4;
            // Get first number.
            let mut a = 0;
            let max_digits = cmp::min(3, input.len() - i);
            for _ in 0..max_digits {
                let c = bytes[i];
                let is_digit = c >= b'0' && c <= b'9';
                if is_digit {
                    a = a * 10 + (c - b'0') as i32;
                    i += 1;
                } else {
                    break;
                }
            }
            if a == 0 {
                continue;
            }
            // Get comma.
            if i >= input.len() || bytes[i] != b',' {
                continue;
            }
            i += 1;
            // Get second number.
            let mut b = 0;
            let max_digits = cmp::min(3, input.len() - i);
            for _ in 0..max_digits {
                let c = bytes[i];
                let is_digit = c >= b'0' && c <= b'9';
                if is_digit {
                    b = b * 10 + (c - b'0') as i32;
                    i += 1;
                } else {
                    break;
                }
            }
            if b == 0 {
                continue;
            }
            // Get closing parenthesis.
            if i >= input.len() || bytes[i] != b')' {
                continue;
            }
            i += 1;
            // Save result.
            results.push((a, b));
        } else {
            i += 1;
        }
    }
    return results;
}

pub fn parse_instructions_no_regex_and_sum(input: &str) -> i32 {
    let mut total = 0;
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < input.len() - 4 {
        if bytes[i] == b'm' && bytes[i + 1] == b'u' && bytes[i + 2] == b'l' && bytes[i + 3] == b'(' {
            i += 4;
            // Get first number.
            let mut a = 0;
            let max_digits = cmp::min(3, input.len() - i);
            for _ in 0..max_digits {
                let c = bytes[i];
                let is_digit = c >= b'0' && c <= b'9';
                if is_digit {
                    a = a * 10 + (c - b'0') as i32;
                    i += 1;
                } else {
                    break;
                }
            }
            if a == 0 {
                continue;
            }
            // Get comma.
            if i >= input.len() || bytes[i] != b',' {
                continue;
            }
            i += 1;
            // Get second number.
            let mut b = 0;
            let max_digits = cmp::min(3, input.len() - i);
            for _ in 0..max_digits {
                let c = bytes[i];
                let is_digit = c >= b'0' && c <= b'9';
                if is_digit {
                    b = b * 10 + (c - b'0') as i32;
                    i += 1;
                } else {
                    break;
                }
            }
            if b == 0 {
                continue;
            }
            // Get closing parenthesis.
            if i >= input.len() || bytes[i] != b')' {
                continue;
            }
            i += 1;
            // Sum result.
            total += a * b;
        } else {
            i += 1;
        }
    }
    return total;
}

pub fn parse_instructions_with_do(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut results: Vec<(i32, i32)> = Vec::new();
    let mut do_mul = true;
    for captures in re.captures_iter(input) {
        if let Some(_) = captures.get(3) {
            do_mul = true;
        } else if let Some(_) = captures.get(4) {
            do_mul = false;
        } else if do_mul {
            if let (Some(a), Some(b)) = (captures.get(1), captures.get(2)) {
                if let (Ok(a), Ok(b)) = (a.as_str().parse::<i32>(), b.as_str().parse::<i32>()) {
                    results.push((a, b));
                }
            }
        }
    }
    return results;
}

pub fn parse_instructions_with_do_and_sum(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut total = 0;
    let mut do_mul = true;
    for captures in re.captures_iter(input) {
        if let Some(_) = captures.get(3) {
            do_mul = true;
        } else if let Some(_) = captures.get(4) {
            do_mul = false;
        } else if do_mul {
            if let (Some(a), Some(b)) = (captures.get(1), captures.get(2)) {
                if let (Ok(a), Ok(b)) = (a.as_str().parse::<i32>(), b.as_str().parse::<i32>()) {
                    total += a * b;
                }
            }
        }
    }
    return total;
}

pub fn parse_instructions_with_do_no_regex(input: &str) -> Vec<(i32, i32)> {
    let mut results: Vec<(i32, i32)> = Vec::new();
    let mut do_mul = true;
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < input.len() - 4 {
        if bytes[i] == b'm' && bytes[i + 1] == b'u' && bytes[i + 2] == b'l' && bytes[i + 3] == b'(' {
            i += 4;
            // Get first number.
            let mut a = 0;
            let max_digits = cmp::min(3, input.len() - i);
            for _ in 0..max_digits {
                let c = bytes[i];
                let is_digit = c >= b'0' && c <= b'9';
                if is_digit {
                    a = a * 10 + (c - b'0') as i32;
                    i += 1;
                } else {
                    break;
                }
            }
            if a == 0 {
                continue;
            }
            // Get comma.
            if i >= input.len() || bytes[i] != b',' {
                continue;
            }
            i += 1;
            // Get second number.
            let mut b = 0;
            let max_digits = cmp::min(3, input.len() - i);
            for _ in 0..max_digits {
                let c = bytes[i];
                let is_digit = c >= b'0' && c <= b'9';
                if is_digit {
                    b = b * 10 + (c - b'0') as i32;
                    i += 1;
                } else {
                    break;
                }
            }
            if b == 0 {
                continue;
            }
            // Get closing parenthesis.
            if i >= input.len() || bytes[i] != b')' {
                continue;
            }
            i += 1;
            // Save result.
            if do_mul {
                results.push((a, b));
            }
        } else if bytes[i] == b'd' && bytes[i + 1] == b'o' && bytes[i + 2] == b'(' && bytes[i + 3] == b')' {
            i += 4;
            do_mul = true;
        } else if i < input.len() - 7
            && bytes[i] == b'd'
            && bytes[i + 1] == b'o'
            && bytes[i + 2] == b'n'
            && bytes[i + 3] == b'\''
            && bytes[i + 4] == b't'
            && bytes[i + 5] == b'('
            && bytes[i + 6] == b')'
        {
            i += 7;
            do_mul = false;
        } else {
            i += 1;
        }
    }
    return results;
}

pub fn parse_instructions_with_do_no_regex_and_sum(input: &str) -> i32 {
    let mut total = 0;
    let mut do_mul = true;
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < input.len() - 4 {
        if bytes[i] == b'm' && bytes[i + 1] == b'u' && bytes[i + 2] == b'l' && bytes[i + 3] == b'(' {
            i += 4;
            // Get first number.
            let mut a = 0;
            let max_digits = cmp::min(3, input.len() - i);
            for _ in 0..max_digits {
                let c = bytes[i];
                let is_digit = c >= b'0' && c <= b'9';
                if is_digit {
                    a = a * 10 + (c - b'0') as i32;
                    i += 1;
                } else {
                    break;
                }
            }
            if a == 0 {
                continue;
            }
            // Get comma.
            if i >= input.len() || bytes[i] != b',' {
                continue;
            }
            i += 1;
            // Get second number.
            let mut b = 0;
            let max_digits = cmp::min(3, input.len() - i);
            for _ in 0..max_digits {
                let c = bytes[i];
                let is_digit = c >= b'0' && c <= b'9';
                if is_digit {
                    b = b * 10 + (c - b'0') as i32;
                    i += 1;
                } else {
                    break;
                }
            }
            if b == 0 {
                continue;
            }
            // Get closing parenthesis.
            if i >= input.len() || bytes[i] != b')' {
                continue;
            }
            i += 1;
            // Sum result.
            if do_mul {
                total += a * b;
            }
        } else if bytes[i] == b'd' && bytes[i + 1] == b'o' && bytes[i + 2] == b'(' && bytes[i + 3] == b')' {
            i += 4;
            do_mul = true;
        } else if i < input.len() - 7
            && bytes[i] == b'd'
            && bytes[i + 1] == b'o'
            && bytes[i + 2] == b'n'
            && bytes[i + 3] == b'\''
            && bytes[i + 4] == b't'
            && bytes[i + 5] == b'('
            && bytes[i + 6] == b')'
        {
            i += 7;
            do_mul = false;
        } else {
            i += 1;
        }
    }
    return total;
}

pub fn sum_instructions(instructions: &[(i32, i32)]) -> i32 {
    let mut total = 0;
    for (a, b) in instructions {
        total += a * b;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_DO_INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse_instructions() {
        assert_eq!(parse_instructions(""), []);
        assert_eq!(parse_instructions("mul(1,2)"), [(1, 2)]);
    }

    #[test]
    fn test_parse_instructions_example() {
        assert_eq!(parse_instructions(EXAMPLE_INPUT), [(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test_parse_instructions_and_sum_example() {
        assert_eq!(parse_instructions_and_sum(EXAMPLE_INPUT), 161);
    }

    #[test]
    fn test_parse_instructions_no_regex_example() {
        assert_eq!(parse_instructions_no_regex(EXAMPLE_INPUT), [(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test_parse_instructions_no_regex_and_sum_example() {
        assert_eq!(parse_instructions_no_regex_and_sum(EXAMPLE_INPUT), 161);
    }

    #[test]
    fn test_parse_instructions_with_do() {
        assert_eq!(parse_instructions_with_do("mul(1,2)don't()mul(3,4)"), [(1, 2)]);
        assert_eq!(parse_instructions_with_do("mul(1,2)don't()mul(3,4)do()mul(5,6)"), [(1, 2), (5, 6)]);
    }

    #[test]
    fn test_parse_instructions_with_do_example() {
        assert_eq!(parse_instructions_with_do(EXAMPLE_DO_INPUT), [(2, 4), (8, 5)]);
    }

    #[test]
    fn test_parse_instructions_with_do_and_sum_example() {
        assert_eq!(parse_instructions_with_do_and_sum(EXAMPLE_DO_INPUT), 48);
    }

    #[test]
    fn test_parse_instructions_with_do_no_regex_example() {
        assert_eq!(parse_instructions_with_do_no_regex(EXAMPLE_DO_INPUT), [(2, 4), (8, 5)]);
    }

    #[test]
    fn test_parse_instructions_with_do_no_regex_and_sum_example() {
        assert_eq!(parse_instructions_with_do_no_regex_and_sum(EXAMPLE_DO_INPUT), 48);
    }

    #[test]
    fn test_sum_instructions() {
        let instructions = [(1, 2), (3, 4), (5, 6)];
        assert_eq!(sum_instructions(&instructions), 44);
    }

    #[test]
    fn test_sum_instructions_example() {
        let instructions = [(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(sum_instructions(&instructions), 161);
    }
}
