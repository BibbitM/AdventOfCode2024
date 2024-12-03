use std::fs::File;
use std::{fs, io};

mod day_01;
mod day_02;
mod day_03;

fn main() -> io::Result<()> {
    println!("# Advent Of Code 2021 results");

    // Day 01
    {
        let input = "inputs\\day_01.txt";
        let file = match File::open(&input) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: Failed to open file '{}': {}", input, e);
                return Err(e);
            }
        };

        let reader = io::BufReader::new(file);
        let (array1, array2) = day_01::parse_numbers(reader);

        let total_distance = day_01::sum_distances(&array1, &array2);
        println!("Day 01: The total distance is **{}**  ", total_distance);

        let total_similarity = day_01::sum_similarities(&array1, &array2);
        println!("Day 01: The similarity score is **{}**  ", total_similarity);
    }

    // Day 02
    {
        let input = "inputs\\day_02.txt";
        let file = match File::open(&input) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: Failed to open file '{}': {}", input, e);
                return Err(e);
            }
        };

        let reader = io::BufReader::new(file);
        let reports = day_02::parse_reports(reader);

        let safe_reports = day_02::count_safe_reports(&reports);
        println!("Day 02: The number of safe reports is **{}**  ", safe_reports);

        let safe_reports_with_tolerance = day_02::count_safe_reports_with_tolerance(&reports);
        println!("Day 02: The number of safe reports with tolerance is **{}**  ", safe_reports_with_tolerance);
    }

    // Day 03
    {
        let input = "inputs\\day_03.txt";
        let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_03.txt'");

        let instructions = day_03::parse_instructions(&input);

        let sum = day_03::sum_instructions(&instructions);
        println!("Day 03: The sum of multiplication of instructions is **{}**  ", sum);
    }

    Ok(())
}
