use regex::Regex;

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

    #[test]
    fn test_parse_instructions() {
        assert_eq!(parse_instructions(""), []);
        assert_eq!(parse_instructions("mul(1,2)"), [(1, 2)]);
    }

    #[test]
    fn test_parse_instructions_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse_instructions(input), [(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test_parse_instructions_with_do() {
        assert_eq!(parse_instructions_with_do("mul(1,2)don't()mul(3,4)"), [(1, 2)]);
        assert_eq!(parse_instructions_with_do("mul(1,2)don't()mul(3,4)do()mul(5,6)"), [(1, 2), (5, 6)]);
    }

    #[test]
    fn test_parse_instructions_with_do_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(parse_instructions_with_do(input), [(2, 4), (8, 5)]);
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
