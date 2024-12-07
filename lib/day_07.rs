use std::io::BufRead;

pub struct Equation {
    value: i64,
    operands: Vec<i32>,
}

impl Equation {
    pub fn new(value: i64, operands: Vec<i32>) -> Equation {
        Equation { value, operands }
    }
}

pub fn parse_equations<R: BufRead>(reader: R) -> Vec<Equation> {
    let mut equations = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(":");
        let value_part = parts.next().unwrap().trim();
        let operands_part = parts.next().unwrap().trim();

        let value = value_part.parse::<i64>().unwrap();
        let operands: Vec<i32> = operands_part.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

        equations.push(Equation::new(value, operands));
    }

    return equations;
}

pub fn can_calibrate_equation(equation: &Equation) -> bool {
    let combined = equation.operands[0] as i64;
    if combined > equation.value {
        return false;
    }
    if equation.operands.len() == 1 {
        return combined == equation.value;
    }

    return can_calibrate_equation_operator(&equation, combined, 1);
}

fn can_calibrate_equation_operator(equation: &Equation, combined: i64, opt_idx: usize) -> bool {
    let operand = equation.operands[opt_idx] as i64;
    // Check multiply operator.
    let combined_mul = combined * operand;
    if combined_mul <= equation.value && opt_idx + 1 < equation.operands.len() {
        if can_calibrate_equation_operator(equation, combined_mul, opt_idx + 1) {
            return true;
        }
    } else if combined_mul == equation.value && opt_idx + 1 == equation.operands.len() {
        return true;
    }
    // Check add operator.
    let combined_add = combined + operand;
    if combined_add <= equation.value && opt_idx + 1 < equation.operands.len() {
        if can_calibrate_equation_operator(equation, combined_add, opt_idx + 1) {
            return true;
        }
    } else if combined_add == equation.value && opt_idx + 1 == equation.operands.len() {
        return true;
    }

    return false;
}

pub fn can_calibrate_equation_concat(equation: &Equation) -> bool {
    let combined = equation.operands[0] as i64;
    if combined > equation.value {
        return false;
    }
    if equation.operands.len() == 1 {
        return combined == equation.value;
    }

    return can_calibrate_equation_concat_operator(&equation, combined, 1);
}

#[inline]
fn concatenate_numbers(a: i64, b: i64) -> i64 {
    let mut mul = 1;
    while mul <= b {
        mul *= 10;
    }
    return a * mul + b;
}

fn can_calibrate_equation_concat_operator(equation: &Equation, combined: i64, opt_idx: usize) -> bool {
    let operand = equation.operands[opt_idx] as i64;
    // Check concatenation operator.
    let combined_concat = concatenate_numbers(combined, operand);
    if combined_concat <= equation.value && opt_idx + 1 < equation.operands.len() {
        if can_calibrate_equation_concat_operator(equation, combined_concat, opt_idx + 1) {
            return true;
        }
    } else if combined_concat == equation.value && opt_idx + 1 == equation.operands.len() {
        return true;
    }
    // Check multiply operator.
    let combined_mul = combined * operand;
    if combined_mul <= equation.value && opt_idx + 1 < equation.operands.len() {
        if can_calibrate_equation_concat_operator(equation, combined_mul, opt_idx + 1) {
            return true;
        }
    } else if combined_mul == equation.value && opt_idx + 1 == equation.operands.len() {
        return true;
    }
    // Check add operator.
    let combined_add = combined + operand;
    if combined_add <= equation.value && opt_idx + 1 < equation.operands.len() {
        if can_calibrate_equation_concat_operator(equation, combined_add, opt_idx + 1) {
            return true;
        }
    } else if combined_add == equation.value && opt_idx + 1 == equation.operands.len() {
        return true;
    }

    return false;
}

pub fn sum_can_calibrate_values(equations: &Vec<Equation>) -> i64 {
    let mut sum = 0;
    for equation in equations {
        if can_calibrate_equation(&equation) {
            sum += equation.value;
        }
    }

    return sum;
}

pub fn sum_can_calibrate_values_concat(equations: &Vec<Equation>) -> i64 {
    let mut sum = 0;
    for equation in equations {
        if can_calibrate_equation_concat(&equation) {
            sum += equation.value;
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE_DATA: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    #[test]
    fn test_parse_equations() {
        let data = "190: 10 19\n3267: 81 40 27\n";
        let cursor = Cursor::new(data);
        let equations = parse_equations(cursor);

        assert_eq!(equations.len(), 2);
        assert_eq!(equations[0].value, 190);
        assert_eq!(equations[0].operands, vec![10, 19]);
        assert_eq!(equations[1].value, 3267);
        assert_eq!(equations[1].operands, vec![81, 40, 27]);
    }

    #[test]
    fn test_can_calibrate_equation_example() {
        // 190: 10 19
        let equation = Equation::new(190, vec![10, 19]);
        let result = can_calibrate_equation(&equation);
        assert_eq!(result, true);
        // 3267: 81 40 27
        let equation = Equation::new(3267, vec![81, 40, 27]);
        let result = can_calibrate_equation(&equation);
        assert_eq!(result, true);
        // 292: 11 6 16 20
        let equation = Equation::new(292, vec![11, 6, 16, 20]);
        let result = can_calibrate_equation(&equation);
        assert_eq!(result, true);

        // 83: 17 5
        let equation = Equation::new(83, vec![17, 5]);
        let result = can_calibrate_equation(&equation);
        assert_eq!(result, false);
        // 156: 15 6
        let equation = Equation::new(156, vec![15, 6]);
        let result = can_calibrate_equation(&equation);
        assert_eq!(result, false);
        // 7290: 6 8 6 15
        let equation = Equation::new(7290, vec![6, 8, 6, 15]);
        let result = can_calibrate_equation(&equation);
        assert_eq!(result, false);
        // 161011: 16 10 13
        let equation = Equation::new(161011, vec![16, 10, 13]);
        let result = can_calibrate_equation(&equation);
        assert_eq!(result, false);
        // 192: 17 8 14
        let equation = Equation::new(192, vec![17, 8, 14]);
        let result = can_calibrate_equation(&equation);
        assert_eq!(result, false);
        // 21037: 9 7 18 13
        let equation = Equation::new(21037, vec![9, 7, 18, 13]);
        let result = can_calibrate_equation(&equation);
        assert_eq!(result, false);
    }

    #[test]
    fn test_can_calibrate_equation_concat_example() {
        // 156: 15 6
        let equation = Equation::new(156, vec![15, 6]);
        let result = can_calibrate_equation_concat(&equation);
        assert_eq!(result, true);
        // 7290: 6 8 6 15
        let equation = Equation::new(7290, vec![6, 8, 6, 15]);
        let result = can_calibrate_equation_concat(&equation);
        assert_eq!(result, true);
        // 192: 17 8 14
        let equation = Equation::new(192, vec![17, 8, 14]);
        let result = can_calibrate_equation_concat(&equation);
        assert_eq!(result, true);

        //15: 8 2 5
        let equation = Equation::new(15, vec![8, 2, 5]);
        let result = can_calibrate_equation_concat(&equation);
        assert_eq!(result, true);
    }

    #[test]
    fn test_sum_can_calibrate_values_example() {
        let cursor = Cursor::new(EXAMPLE_DATA);
        let equations = parse_equations(cursor);
        assert_eq!(sum_can_calibrate_values(&equations), 3749);
    }

    #[test]
    fn test_sum_can_calibrate_values_concat_example() {
        let cursor = Cursor::new(EXAMPLE_DATA);
        let equations = parse_equations(cursor);
        assert_eq!(sum_can_calibrate_values_concat(&equations), 11387);
    }
}
