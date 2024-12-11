pub fn parse_stones(input: &String) -> Vec<u64> {
    input.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn num_decimal_digits(value: u64) -> u32 {
    if value == 0 {
        1 // Special case for 0
    } else {
        (value as f64).log10().floor() as u32 + 1
    }
    // Check what is better
    //t mut value = value;
    // let mut digits = 0;
    // while value 10000 {
    //     digits += 4;
    //     value /= 10000;
    // }
    // while value > 0 {
    //     digits += 1;
    //     value /= 10;
    // }
    // digits
}

fn split_stone(stone: u64, split: u32) -> u32 {
    let num_digits = num_decimal_digits(stone);
    if split == 1 {
        if stone != 0 && num_digits % 2 == 0 {
            return 2;
        } else {
            return 1;
        }
    }

    if stone == 0 {
        return split_stone(1, split - 1);
    } else if num_digits % 2 == 0 {
        return split_stone(stone / 10_u64.pow(num_digits / 2), split - 1) + split_stone(stone % 10_u64.pow(num_digits / 2), split - 1);
    } else {
        return split_stone(stone * 2024, split - 1);
    }
}

pub fn calculate_number_of_stones_after_blinks(stones: &Vec<u64>, blinks: u32) -> u32 {
    let mut total_stones = 0;
    for stone in stones {
        total_stones += split_stone(*stone, blinks);
    }
    return total_stones;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "125 17";

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_stones(&EXAMPLE_INPUT.to_string()), vec![125, 17]);
    }

    #[test]
    fn test_num_decimal_digits() {
        assert_eq!(num_decimal_digits(0), 1);
        assert_eq!(num_decimal_digits(1), 1);
        assert_eq!(num_decimal_digits(10), 2);
        assert_eq!(num_decimal_digits(100), 3);
        assert_eq!(num_decimal_digits(910012), 6);
    }

    #[test]
    fn test_spilit_stone() {
        assert_eq!(split_stone(0, 1), 1);
        assert_eq!(split_stone(1, 1), 1);
        assert_eq!(split_stone(1024, 1), 2);
        assert_eq!(split_stone(1024, 2), 4);
        assert_eq!(split_stone(1024, 3), 4);
    }

    #[test]
    fn test_calculate_number_of_stones_after_blinks_example() {
        let stones = parse_stones(&EXAMPLE_INPUT.to_string());
        assert_eq!(calculate_number_of_stones_after_blinks(&stones, 6), 22);
        assert_eq!(calculate_number_of_stones_after_blinks(&stones, 25), 55312);
    }
}
