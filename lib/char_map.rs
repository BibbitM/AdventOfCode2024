pub struct CharMap {
    map: Vec<char>,
    pub width: i32,
    pub height: i32,
    end_width: i32,
}

impl CharMap {
    pub fn new(input: String) -> CharMap {
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
        CharMap {
            map: input.chars().collect(),
            width,
            height,
            end_width,
        }
    }
    pub fn clone(&self) -> CharMap {
        CharMap {
            map: self.map.clone(),
            width: self.width,
            height: self.height,
            end_width: self.end_width,
        }
    }

    pub fn assign(&mut self, other: &CharMap) {
        //self.map.clone_from(&other.map);
        for i in 0..self.map.len() {
            self.map[i] = other.map[i];
        }
        self.width = other.width;
        self.height = other.height;
        self.end_width = other.end_width;
    }

    #[inline]
    pub fn get(&self, x: i32, y: i32) -> char {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return 0 as char;
        }
        return self.map[(y * (self.width + self.end_width) + x) as usize];
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, value: char) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        let pos = (y * (self.width + self.end_width) + x) as usize;
        self.map[pos] = value;
    }
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

    #[test]
    fn test_get_size() {
        let guard_map = CharMap::new(INPUT.to_string());
        assert_eq!(guard_map.width, 6);
        assert_eq!(guard_map.height, 5);
    }

    #[test]
    fn test_get() {
        let guard_map = CharMap::new(INPUT.to_string());
        assert_eq!(guard_map.get(0, 0), '.');
        assert_eq!(guard_map.get(2, 0), '#');
        assert_eq!(guard_map.get(5, 0), '.');
        assert_eq!(guard_map.get(0, 3), '#');
        assert_eq!(guard_map.get(5, 4), '.');
    }

    #[test]
    fn test_get_outside_bounds() {
        let guard_map = CharMap::new(INPUT.to_string());
        assert_eq!(guard_map.get(6, 0), 0 as char);
        assert_eq!(guard_map.get(0, 5), 0 as char);
        assert_eq!(guard_map.get(6, 5), 0 as char);

        assert_eq!(guard_map.get(-1, 0), 0 as char);
        assert_eq!(guard_map.get(0, -10), 0 as char);
        assert_eq!(guard_map.get(-10, -1), 0 as char);
    }

    #[test]
    fn test_set() {
        let mut guard_map = CharMap::new(INPUT.to_string());
        guard_map.set(0, 0, 'X');
        assert_eq!(guard_map.get(0, 0), 'X');
        guard_map.set(2, 2, 'X');
        assert_eq!(guard_map.get(2, 2), 'X');
    }
}
