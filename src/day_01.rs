use std::io::BufRead;

pub fn calculate_distance(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

pub fn calulate_similarity(a: i32, count1: i32, count2: i32) -> i32 {
    a * count1 * count2
}

pub fn sum_distances(array1: &[i32], array2: &[i32]) -> i32 {
    let mut sorted_array1 = array1.to_vec();
    let mut sorted_array2 = array2.to_vec();
    sorted_array1.sort();
    sorted_array2.sort();

    let mut total_distance = 0;
    let len = sorted_array1.len().min(sorted_array2.len());
    for i in 0..len {
        total_distance += calculate_distance(sorted_array1[i], sorted_array2[i]);
    }
    total_distance
}

pub fn sum_similarities(array1: &[i32], array2: &[i32]) -> i32 {
    let mut sorted_array1 = array1.to_vec();
    let mut sorted_array2 = array2.to_vec();
    sorted_array1.sort();
    sorted_array2.sort();

    let mut index1 = 0;
    let mut index2 = 0;

    let mut total_similarity = 0;

    while index1 < sorted_array1.len() && index2 < sorted_array2.len() {
        if sorted_array1[index1] < sorted_array2[index2] {
            index1 += 1;
        } else if sorted_array1[index1] > sorted_array2[index2] {
            index2 += 1;
        } else {
            let number = sorted_array1[index1];
            let mut count1 = 0;
            while index1 < sorted_array1.len() && sorted_array1[index1] == number {
                count1 += 1;
                index1 += 1;
            }
            let mut count2 = 0;
            while index2 < sorted_array2.len() && sorted_array2[index2] == number {
                count2 += 1;
                index2 += 1;
            }

            total_similarity += calulate_similarity(number, count1, count2);
        }
    }

    total_similarity
}

pub fn parse_numbers<R: BufRead>(reader: R) -> (Vec<i32>, Vec<i32>) {
    let mut array1 = Vec::new();
    let mut array2 = Vec::new();

    // Iterate over each line in the reader
    for line in reader.lines() {
        if let Ok(line_content) = line {
            // Split the line into parts and parse the numbers
            let parts: Vec<&str> = line_content.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(first), Ok(second)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    // Push the numbers into their respective arrays
                    array1.push(first);
                    array2.push(second);
                }
            }
        }
    }

    (array1, array2)
}


// Test the function
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_calculate_distance() {
        assert_eq!(calculate_distance(5, 3), 2);
        assert_eq!(calculate_distance(3, 5), 2);
        assert_eq!(calculate_distance(-4, 6), 10);
        assert_eq!(calculate_distance(10, 10), 0);
    }

    #[test]
    fn test_calculate_similarity() {
        assert_eq!(calulate_similarity(5, 3, 2), 30);
        assert_eq!(calulate_similarity(3, 5, 2), 30);
        assert_eq!(calulate_similarity(-4, 6, 10), -240);
        assert_eq!(calulate_similarity(10, 10, 0), 0);
    }

    #[test]
    fn test_sum_distances() {
        let array1 = [1, 2, 3];
        let array2 = [4, 5, 6];
        assert_eq!(sum_distances(&array1, &array2), 9);

        let array1 = [-1, -2, -3];
        let array2 = [1, 2, 3];
        assert_eq!(sum_distances(&array1, &array2), 12);

        let array1 = [3, 1, 2];
        let array2 = [6, 5, 4];
        assert_eq!(sum_distances(&array1, &array2), 9);
    }

    #[test]
    fn test_sum_distances_example() {
        let array1 = [3, 4, 2, 1, 3, 3];
        let array2 = [4, 3, 5, 3, 9, 3];
        assert_eq!(sum_distances(&array1, &array2), 11);
    }

    #[test]
    fn test_sum_similarities() {
        let array1 = [1, 2, 3];
        let array2 = [4, 5, 6];
        assert_eq!(sum_similarities(&array1, &array2), 0);

        let array1 = [1, 2, 3, 3, 3];
        let array2 = [3, 3, 3, 4, 5, 6];
        assert_eq!(sum_similarities(&array1, &array2), 27);

        let array1 = [1, 2, 3, 3, 3];
        let array2 = [3, 3, 3, 4, 5, 6, 6, 6];
        assert_eq!(sum_similarities(&array1, &array2), 27);

        let array1 = [1, 2, 3, 3, 3];
        let array2 = [3, 3, 3, 4, 5, 6, 6, 6, 6, 6];
        assert_eq!(sum_similarities(&array1, &array2), 27);

        let array1 = [1, 2, 3, 3, 3];
        let array2 = [3, 3, 3, 4, 5, 6, 6, 6, 6, 6, 6, 6];
        assert_eq!(sum_similarities(&array1, &array2), 27);

        let array1 = [1, 2, 3, 3, 3];
        let array2 = [3, 3, 3, 4, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6];
        assert_eq!(sum_similarities(&array1, &array2), 27);
    }

    #[test]
    fn test_sum_similarities_example() {
        let array1 = [3, 4, 2, 1, 3, 3];
        let array2 = [4, 3, 5, 3, 9, 3];
        assert_eq!(sum_similarities(&array1, &array2), 31);
    }

    #[test]
    fn test_parse_numbers() {
        let data = "1 2\n3 4\n5 6\n";
        let cursor = Cursor::new(data);
        let (first_numbers, second_numbers) = parse_numbers(cursor);

        assert_eq!(first_numbers, vec![1, 3, 5]);
        assert_eq!(second_numbers, vec![2, 4, 6]);
    }
}
