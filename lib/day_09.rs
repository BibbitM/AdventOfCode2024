
pub fn calculate_filesystem_checksum(disk_map: &String) -> usize {
    let mut checksum = 0;

    #[inline]
    fn get_disk_len(disk_map: &String, idx: usize) -> usize {
        let c = disk_map.chars().nth(idx).unwrap();
        return c as usize - '0' as usize;
    }
    #[inline]
    fn get_file_id(file_idx: usize) -> usize {
        return file_idx / 2;
    }

    fn sum_of_sequence(n: usize, count: usize) -> usize {
        // Calculate the last term in the sequence.
        let last_term = n + count - 1;
        // Calculate the number of terms.
        let terms = count;
        // Apply the arithmetic series formula.
        terms * (n + last_term) / 2
    }

    #[inline]
    fn calculate_checksum(file_id: usize, block_idx: usize, block_count: usize) -> usize {
        return file_id * sum_of_sequence(block_idx, block_count);
    }

    // At even indexes, the value is file length.
    // At odd indexes, the value is free space.
    let mut last_file_idx = disk_map.len() - 1;
    last_file_idx = last_file_idx & !1; //< Bitwise AND with 11111110 to clear the last bit.
    let mut last_file_length = get_disk_len(disk_map, last_file_idx);

    let mut current_file_idx = 0;

    let mut block_position = 0;

    while current_file_idx < last_file_idx {
        // If it is a file then calculate the checksum.
        if current_file_idx % 2 == 0 {
            let current_file_length = get_disk_len(disk_map, current_file_idx);
            checksum += calculate_checksum(get_file_id(current_file_idx), block_position, current_file_length);
            block_position += current_file_length;
            current_file_idx += 1;
        } else {
            // It is a odd number, so it is a free space.
            let mut current_free_space = get_disk_len(disk_map, current_file_idx);
            // Put the last file in the free space.
            while current_free_space > 0 && current_file_idx < last_file_idx {
                let last_file_to_put_in_free_space = std::cmp::min(current_free_space, last_file_length);
                checksum += calculate_checksum(get_file_id(last_file_idx), block_position, last_file_to_put_in_free_space);
                block_position += last_file_to_put_in_free_space;

                current_free_space -= last_file_to_put_in_free_space;
                last_file_length -= last_file_to_put_in_free_space;
                if last_file_length == 0 {
                    last_file_idx -= 2;
                    last_file_length = get_disk_len(disk_map, last_file_idx);
                }
            }
            current_file_idx += 1;
        }
    }

    if last_file_idx == current_file_idx {
        checksum += calculate_checksum(get_file_id(last_file_idx), block_position, last_file_length);
        // block_position += last_file_length; //< Not needed, last file length is the last block.
    }

    return checksum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_calculate_filesystem_checksum_example() {
        let input = EXAMPLE_INPUT.to_string();
        assert_eq!(calculate_filesystem_checksum(&input), 1928);
    }
}
