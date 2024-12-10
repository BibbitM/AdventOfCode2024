pub fn calculate_filesystem_checksum(disk_map: &Vec<char>) -> usize {
    #[inline]
    fn get_disk_len(disk_map: &Vec<char>, idx: usize) -> usize {
        let c = disk_map[idx];
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

    let mut checksum = 0;

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

pub fn calculate_filesystem_checksum_v2(disk_map: &Vec<char>) -> usize {
    let mut file_map = Vec::new();

    #[inline]
    fn get_disk_len(disk_map: &Vec<char>, idx: usize) -> u32 {
        let c = disk_map[idx];
        return c as u32 - '0' as u32;
    }

    fn sum_of_sequence(n: u32, count: u32) -> u32 {
        // Calculate the last term in the sequence.
        let last_term = n + count - 1;
        // Calculate the number of terms.
        let terms = count;
        // Apply the arithmetic series formula.
        terms * (n + last_term) / 2
    }

    fn calculate_checksum(file_id: usize, block_idx: u32, block_count: u32) -> usize {
        return file_id * (sum_of_sequence(block_idx, block_count) as usize);
    }

    let last_file_id = (disk_map.len() + 1) / 2;
    for file_id in 0..last_file_id {
        let file_len = get_disk_len(disk_map, file_id * 2);
        let free_space_len = if file_id * 2 + 1 < disk_map.len() { get_disk_len(disk_map, file_id * 2 + 1) } else { 0 };
        file_map.push((file_id, file_len, free_space_len));
    }

    let mut file_idx = file_map.len() - 1;
    let mut file_id = file_map[file_idx].0;
    while file_idx > 0 && file_id > 0 {
        if file_map[file_idx].0 != file_id {
            file_idx -= 1;
            continue;
        }

        let file_len = file_map[file_idx].1;

        for i in 0..file_idx {
            let free_space = file_map[i].2;
            if free_space >= file_len {
                let free_space_left = free_space - file_len;
                file_map[i].2 = 0;
                file_map.insert(i + 1, (file_map[file_idx].0, file_map[file_idx].1, free_space_left));
                // We have inserted the file before the file index. We have to increase it by 1 to get it.
                file_idx += 1;
                // The previous file free space have to increased by the file length and remaining free space.
                file_map[file_idx - 1].2 += file_map[file_idx].1 + file_map[file_idx].2;
                // Everything is copied. We can remove the file from the list.
                file_map.remove(file_idx);
                break;
            }
        }

        file_idx -= 1;
        file_id -= 1;
    }

    let mut checksum = 0;
    let mut block_position = 0;

    for i in 0..file_map.len() {
        let file_id = file_map[i].0;
        let file_len = file_map[i].1;
        let free_space = file_map[i].2;
        checksum += calculate_checksum(file_id, block_position, file_len);

        block_position += file_len;
        block_position += free_space;
    }

    return checksum;
}

pub fn calculate_filesystem_checksum_v2_optimized(disk_map: &Vec<char>) -> usize {
    struct File {
        id: usize,
        pos: u32,
        len: u32,
    }
    struct FreeSpace {
        pos: u32,
        len: u32,
    }

    #[inline]
    fn get_disk_len(disk_map: &Vec<char>, idx: usize) -> u32 {
        let c = disk_map[idx];
        return c as u32 - '0' as u32;
    }

    #[inline]
    fn sum_of_sequence(n: u32, count: u32) -> u32 {
        // Calculate the last term in the sequence.
        let last_term = n + count - 1;
        // Calculate the number of terms.
        let terms = count;
        // Apply the arithmetic series formula.
        terms * (n + last_term) / 2
    }

    #[inline]
    fn calculate_checksum(file_id: usize, block_idx: u32, block_count: u32) -> usize {
        return file_id * (sum_of_sequence(block_idx, block_count) as usize);
    }

    fn find_first_free_space(free_spaces: &Vec<FreeSpace>, start_i: usize, len: u32) -> usize {
        for i in start_i..free_spaces.len() {
            if i > 0 {
                assert!(free_spaces[i - 1].pos + free_spaces[i - 1].len <= free_spaces[i].pos);
            }
            if free_spaces[i].len == len {
                return i;
            }
        }
        // Return npos if not found.
        return free_spaces.len();
    }

    // Get all files and free spaces.
    let mut files = Vec::new();
    let mut free_spaces = Vec::new();

    let last_file_id = (disk_map.len() + 1) / 2;
    let mut file_pos = 0;
    for file_id in 0..last_file_id {
        let file_len = get_disk_len(disk_map, file_id * 2);
        files.push(File {
            id: file_id,
            pos: file_pos,
            len: file_len,
        });
        file_pos += file_len;

        let free_space_len = if file_id * 2 + 1 < disk_map.len() { get_disk_len(disk_map, file_id * 2 + 1) } else { 0 };
        if free_space_len > 0 {
            free_spaces.push(FreeSpace {
                pos: file_pos,
                len: free_space_len,
            });
        }
        file_pos += free_space_len;
    }

    // Find first free space for size 1-9
    let mut first_free_space_pos: [usize; 10] = [0; 10];
    for i in 0u32..10 {
        first_free_space_pos[i as usize] = find_first_free_space(&free_spaces, 0, i);
    }

    // Process in loop all files starting from the last.
    for file in files.iter_mut().rev() {
        // Find the first free space for the file length.
        let mut i_best = 0;
        let mut best_pos = file.pos;
        for i in file.len..10 {
            if first_free_space_pos[i as usize] >= free_spaces.len() {
                continue;
            }
            let free_space = &free_spaces[first_free_space_pos[i as usize]];
            if free_space.pos < best_pos {
                i_best = i;
                best_pos = free_space.pos;
            }
        }

        if i_best > 0 {
            let free_space = &mut free_spaces[first_free_space_pos[i_best as usize]];
            // Move file to the free space.
            file.pos = free_space.pos;

            // Update the free space.
            free_space.pos += file.len;
            free_space.len -= file.len;

            // Update the first free space for changed and current.
            if free_space.len > 0 && first_free_space_pos[free_space.len as usize] > first_free_space_pos[i_best as usize] {
                first_free_space_pos[free_space.len as usize] = first_free_space_pos[i_best as usize];
            }

            first_free_space_pos[i_best as usize] = find_first_free_space(&free_spaces, first_free_space_pos[i_best as usize], i_best);
        }
    }

    // Calculate the checksum.
    let mut checksum = 0;
    for file in files {
        checksum += calculate_checksum(file.id, file.pos, file.len);
    }

    return checksum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_calculate_filesystem_checksum_example() {
        let input = EXAMPLE_INPUT.to_string().chars().collect();
        assert_eq!(calculate_filesystem_checksum(&input), 1928);
    }

    #[test]
    fn test_calculate_filesystem_checksum_v2_example() {
        let input = EXAMPLE_INPUT.to_string().chars().collect();
        assert_eq!(calculate_filesystem_checksum_v2(&input), 2858);
    }

    #[test]
    fn test_calculate_filesystem_checksum_v2_optimized_example() {
        let input = EXAMPLE_INPUT.to_string().chars().collect();
        assert_eq!(calculate_filesystem_checksum_v2_optimized(&input), 2858);
    }
}
