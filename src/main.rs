use advent::char_map::CharMap;
use advent::{day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08};
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

        let sum = day_03::parse_instructions_no_regex_and_sum(&input);
        println!("Day 03: The sum of multiplication of instructions is **{}**  ", sum);

        let sum_with_do = day_03::parse_instructions_with_do_no_regex_and_sum(&input);
        println!("Day 03: The sum of multiplication of instructions of just enabled multiplications is **{}**  ", sum_with_do);
    }

    // Day 04
    {
        let input = "inputs\\day_04.txt";
        let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_04.txt'");

        let word_search = day_04::WordSearch::new(input);

        let xmas_count = word_search.count_xmas_check_xs();
        println!("Day 04: The word XMAS appears **{}** times  ", xmas_count);

        let mas_diagonal_count = word_search.count_mas_diagonal_check_mask();
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

    // Day 06
    {
        let input = "inputs\\day_06.txt";
        let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_06.txt'");

        let mut guard_map = CharMap::new(input);
        let mut guard_map_to_block = guard_map.clone();

        let dinstinct_positions = day_06::move_guard(&mut guard_map);
        println!("Day 06: The guard visited **{}** distinct positions  ", dinstinct_positions);

        let obstruction_positions = day_06::block_guard_assign_map(&mut guard_map_to_block);
        println!("Day 06: The guard can be looped with **{}** obstruction positions  ", obstruction_positions);
    }

    // Day 07
    {
        let input = "inputs\\day_07.txt";
        let file = match File::open(&input) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: Failed to open file '{}': {}", input, e);
                return Err(e);
            }
        };

        let reader = io::BufReader::new(file);
        let equations = day_07::parse_equations(reader);

        let sum = day_07::sum_can_calibrate_values(&equations);
        println!("Day 07: The total calibration result is **{}**  ", sum);

        let sum = day_07::sum_can_calibrate_values_concat(&equations);
        println!("Day 07: The total calibration result with concatenation is **{}**  ", sum);
    }

    // Day 08
    {
        let input = "inputs\\day_08.txt";
        let input = fs::read_to_string(input).expect("Error: Failed to read file 'inputs\\day_08.txt'");

        let antennas_map = CharMap::new(input.to_string());
        let antennas = day_08::gather_antennas(&antennas_map);

        let antinodes = day_08::find_antinodes_sort_dedup(&antennas, &antennas_map);
        println!("Day 08: The map contains **{}** unique antinode locations  ", antinodes.len());

        let antinodes_in_line = day_08::find_antinodes_in_line_sort_dedup(&antennas, &antennas_map);
        println!("Day 08: The map contains **{}** unique antinode locations in line  ", antinodes_in_line.len());
    }

    Ok(())
}
