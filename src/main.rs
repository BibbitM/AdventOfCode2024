use std::fs::File;
use std::io;

mod day_01;

fn main() -> io::Result<()> {

    println!("# AdventOfCode2021 results");

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

        // Print the total distance to the output
        println!("day_01: The total distance is {}  ", total_distance);
    }

    Ok(())
}
