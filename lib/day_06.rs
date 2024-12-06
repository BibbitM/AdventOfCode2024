pub struct GuardMap {
    map: Vec<char>,
    width: i32,
    height: i32,
    end_width: i32,
}

impl GuardMap {
    pub fn new(input: String) -> GuardMap {
        let pos_n = input.find('\n');
        let pos_r = input.find('\r');
        let mut width = 0;
        let mut end_width = 0;
        if pos_n.is_some() && pos_r.is_some() {
            width = std::cmp::min(pos_n.unwrap() as i32, pos_r.unwrap() as i32);
            end_width = 2;
        } else if pos_n.is_some() {
            width = pos_n.unwrap() as i32;
            end_width = 1;
        } else if pos_r.is_some() {
            width = pos_r.unwrap() as i32;
            end_width = 1;
        }
        let height = (input.len() as i32) / (width + end_width);
        GuardMap {
            map: input.chars().collect(),
            width,
            height,
            end_width,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> char {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return 0 as char;
        }
        return self.map[(y * (self.width + self.end_width) + x) as usize];
    }

    pub fn set(&mut self, x: i32, y: i32, value: char) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        let pos = (y * (self.width + self.end_width) + x) as usize;
        self.map[pos] = value;
    }
}

fn find_start_position(guard_map: &GuardMap) -> (i32, i32) {
    for y in 0..guard_map.height {
        for x in 0..guard_map.width {
            if guard_map.get(x, y) == '^' {
                return (x, y);
            }
        }
    }
    return (-1, -1);
}

pub fn move_guard(guard_map: &mut GuardMap) -> usize {
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
    fn test_get_size() {
        let guard_map = GuardMap::new(INPUT.to_string());
        assert_eq!(guard_map.width, 6);
        assert_eq!(guard_map.height, 5);
    }

    #[test]
    fn test_get() {
        let guard_map = GuardMap::new(INPUT.to_string());
        assert_eq!(guard_map.get(0, 0), '.');
        assert_eq!(guard_map.get(2, 0), '#');
        assert_eq!(guard_map.get(5, 0), '.');
        assert_eq!(guard_map.get(0, 3), '#');
        assert_eq!(guard_map.get(5, 4), '.');
    }

    #[test]
    fn test_get_outside_bounds() {
        let guard_map = GuardMap::new(INPUT.to_string());
        assert_eq!(guard_map.get(6, 0), 0 as char);
        assert_eq!(guard_map.get(0, 5), 0 as char);
        assert_eq!(guard_map.get(6, 5), 0 as char);

        assert_eq!(guard_map.get(-1, 0), 0 as char);
        assert_eq!(guard_map.get(0, -10), 0 as char);
        assert_eq!(guard_map.get(-10, -1), 0 as char);
    }

    #[test]
    fn test_set() {
        let mut guard_map = GuardMap::new(INPUT.to_string());
        guard_map.set(0, 0, 'X');
        assert_eq!(guard_map.get(0, 0), 'X');
        guard_map.set(2, 2, 'X');
        assert_eq!(guard_map.get(2, 2), 'X');
    }

    #[test]
    fn test_find_start_position() {
        let guard_map = GuardMap::new(INPUT.to_string());
        let (x, y) = find_start_position(&guard_map);
        assert_eq!(x, 2);
        assert_eq!(y, 2);
    }

    #[test]
    fn test_move_guard() {
        let mut guard_map = GuardMap::new(INPUT.to_string());
        let distinct_positions = move_guard(&mut guard_map);
        assert_eq!(distinct_positions, 12);
    }

    #[test]
    fn test_move_guard_example() {
        let mut guard_map = GuardMap::new(EXAMPLE_INPUT.to_string());
        let distinct_positions = move_guard(&mut guard_map);
        assert_eq!(distinct_positions, 41);
    }
}
