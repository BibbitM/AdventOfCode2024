use advent::{day_01, day_02, day_03, day_04, day_05};
use std::fs::File;
use std::{fs, io};

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

        let instructions_with_do = day_03::parse_instructions_with_do(&input);

        let sum_with_do = day_03::sum_instructions(&instructions_with_do);
        println!("Day 03: The sum of multiplication of instructions of just enabled multiplications is **{}**  ", sum_with_do);
    }

    // Day 04
    {
        let input = "inputs\\day_04.txt";
        let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_04.txt'");

        let word_search = day_04::WordSearch::new(input);

        let xmas_count = word_search.count_xmas();
        println!("Day 04: The word XMAS appears **{}** times  ", xmas_count);

        let mas_diagonal_count = word_search.count_mas_diagonal();
        println!("Day 04: The word MAS appears diagonally **{}** times  ", mas_diagonal_count);
    }

    // Day 05
    {
        let input = "inputs\\day_05.txt";
        let file = match File::open(&input) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: Failed to open file '{}': {}", input, e);
                return Err(e);
            }
        };

        let reader = io::BufReader::new(file);
        let (page_ordering_rules, pages_to_produce) = day_05::parse_pages(reader);

        let sum_of_valid_middle_pages = day_05::sum_of_valid_middle_pages(&page_ordering_rules, &pages_to_produce);
        println!("Day 05: The sum of valid middle pages is **{}**  ", sum_of_valid_middle_pages);

        let mut pages_to_produce = pages_to_produce;
        day_05::correct_pages_to_produce(&page_ordering_rules, &mut pages_to_produce);

        // After correction, all pages are valid
        let sum_of_all_middle_pages = day_05::sum_of_all_middle_pages(&pages_to_produce);
        println!("Day 05: The sum of corrected middle pages is **{}**  ", sum_of_all_middle_pages - sum_of_valid_middle_pages);
    }

    Ok(())
}
