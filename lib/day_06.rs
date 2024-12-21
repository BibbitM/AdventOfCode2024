use crate::char_map::CharMap;

fn find_start_position(guard_map: &CharMap) -> (i32, i32) {
    for y in 0..guard_map.height {
        for x in 0..guard_map.width {
            if guard_map.get(x, y) == '^' {
                return (x, y);
            }
        }
    }
    return (-1, -1);
}

pub fn move_guard(guard_map: &mut CharMap) -> usize {
    let mut distinct_positions = 0;
    let (mut x, mut y) = find_start_position(&guard_map);
    let mut dir_x = 0;
    let mut dir_y = -1;
    while guard_map.get(x, y) != 0 as char {
        if guard_map.get(x, y) != 'X' {
            guard_map.set(x, y, 'X');
            distinct_positions += 1;
        }
        let next_x = x + dir_x;
        let next_y = y + dir_y;
        if guard_map.get(next_x, next_y) != '#' {
            x = next_x;
            y = next_y;
        } else {
            (dir_x, dir_y) = (-dir_y, dir_x);
        }
    }
    return distinct_positions;
}

fn move_guard_check_loop(guard_map: &mut CharMap, x: i32, y: i32, dir_x: i32, dir_y: i32, dir_bit: i32) -> bool {
    let mut x = x;
    let mut y = y;

    let mut dir_x = dir_x;
    let mut dir_y = dir_y;
    let mut dir_bit = dir_bit;

    while guard_map.get(x, y) != 0 as char {
        let pos = guard_map.get(x, y);
        if pos == '.' {
            guard_map.set(x, y, ((1 << dir_bit) as u8) as char);
        } else {
            if (pos as u8) & (1 << dir_bit) != 0 {
                return true;
            }
            guard_map.set(x, y, (pos as u8 | (1 << dir_bit)) as char);
        }
        let next_x = x + dir_x;
        let next_y = y + dir_y;
        if guard_map.get(next_x, next_y) != '#' {
            x = next_x;
            y = next_y;
        } else {
            (dir_x, dir_y) = (-dir_y, dir_x);
            dir_bit = (dir_bit + 1) % 4;
        }
    }

    return false;
}
pub fn block_guard(guard_map: &mut CharMap) -> usize {
    let mut obstruction_positions = 0;
    let (mut x, mut y) = find_start_position(&guard_map);
    // Clear the starting position to simplify the later checks.
    guard_map.set(x, y, '.');
    let mut dir_x = 0;
    let mut dir_y = -1;
    let mut dir_bit = 0;
    while guard_map.get(x, y) != 0 as char {
        let next_x = x + dir_x;
        let next_y = y + dir_y;
        if guard_map.get(next_x, next_y) == '.' {
            let mut guard_map_to_check = guard_map.clone();
            guard_map_to_check.set(next_x, next_y, '#');
            if move_guard_check_loop(&mut guard_map_to_check, x, y, dir_x, dir_y, dir_bit) {
                obstruction_positions += 1;
            }
        }
        let pos = guard_map.get(x, y);
        if pos == '.' {
            guard_map.set(x, y, ((1 << dir_bit) as u8) as char);
        } else {
            guard_map.set(x, y, (pos as u8 | (1 << dir_bit)) as char);
        }
        if guard_map.get(next_x, next_y) != '#' {
            x = next_x;
            y = next_y;
        } else {
            (dir_x, dir_y) = (-dir_y, dir_x);
            dir_bit = (dir_bit + 1) % 4;
        }
    }
    return obstruction_positions;
}

pub fn block_guard_assign_map(guard_map: &mut CharMap) -> usize {
    // Create a copy of the guard map to check for obstructions.
    // We crate it once and reuse it to avoid allocating memory in the loop.
    let mut guard_map_to_check = guard_map.clone();

    let mut obstruction_positions = 0;
    let (mut x, mut y) = find_start_position(&guard_map);
    // Clear the starting position to simplify the later checks.
    guard_map.set(x, y, '.');
    let mut dir_x = 0;
    let mut dir_y = -1;
    let mut dir_bit = 0;
    while guard_map.get(x, y) != 0 as char {
        let next_x = x + dir_x;
        let next_y = y + dir_y;
        if guard_map.get(next_x, next_y) == '.' {
            guard_map_to_check.assign(&guard_map);
            guard_map_to_check.set(next_x, next_y, '#');
            if move_guard_check_loop(&mut guard_map_to_check, x, y, dir_x, dir_y, dir_bit) {
                obstruction_positions += 1;
            }
        }
        let pos = guard_map.get(x, y);
        if pos == '.' {
            guard_map.set(x, y, ((1 << dir_bit) as u8) as char);
        } else {
            guard_map.set(x, y, (pos as u8 | (1 << dir_bit)) as char);
        }
        if guard_map.get(next_x, next_y) != '#' {
            x = next_x;
            y = next_y;
        } else {
            (dir_x, dir_y) = (-dir_y, dir_x);
            dir_bit = (dir_bit + 1) % 4;
        }
    }
    return obstruction_positions;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"..#...
.....#
..^...
#.....
....#.
"#;

    const EXAMPLE_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    #[test]
    fn test_find_start_position() {
        let guard_map = CharMap::new(&INPUT.to_string());
        let (x, y) = find_start_position(&guard_map);
        assert_eq!(x, 2);
        assert_eq!(y, 2);
    }

    #[test]
    fn test_move_guard() {
        let mut guard_map = CharMap::new(&INPUT.to_string());
        let distinct_positions = move_guard(&mut guard_map);
        assert_eq!(distinct_positions, 12);
    }

    #[test]
    fn test_move_guard_example() {
        let mut guard_map = CharMap::new(&EXAMPLE_INPUT.to_string());
        let distinct_positions = move_guard(&mut guard_map);
        assert_eq!(distinct_positions, 41);
    }

    #[test]
    fn test_block_guard_example() {
        let mut guard_map = CharMap::new(&EXAMPLE_INPUT.to_string());
        let obstruction_positions = block_guard(&mut guard_map);
        assert_eq!(obstruction_positions, 6);
    }

    #[test]
    fn test_block_guard_assign_map_example() {
        let mut guard_map = CharMap::new(&EXAMPLE_INPUT.to_string());
        let obstruction_positions = block_guard_assign_map(&mut guard_map);
        assert_eq!(obstruction_positions, 6);
    }
}
