const XMAS: u32 = (('X' as u32) << 0) | (('M' as u32) << 8) | (('A' as u32) << 16) | (('S' as u32) << 24);
const XMAS_REVERSE: u32 = (('S' as u32) << 0) | (('A' as u32) << 8) | (('M' as u32) << 16) | (('X' as u32) << 24);

const MAS: u32 = (('M' as u32) << 0) | (('A' as u32) << 8) | (('S' as u32) << 16);
const MAS_REVERSE: u32 = (('S' as u32) << 0) | (('A' as u32) << 8) | (('M' as u32) << 16);

const MAS_MASK: u32 = (('M' as u32 + 'S' as u32) << 0) | (('M' as u32 + 'S' as u32) << 8);

pub struct WordSearch {
    words: String,
    width: i32,
    height: i32,
}

impl WordSearch {
    pub fn new(words: String) -> WordSearch {
        let width = words.find('\n').unwrap() as i32;
        let height = (words.len() as i32) / (width + 1);
        WordSearch { words, width, height }
    }

    #[inline]
    fn get(&self, x: i32, y: i32) -> char {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return 0 as char;
        }

        return self.words.chars().nth((y * (self.width + 1) + x) as usize).unwrap();
    }

    #[inline]
    fn get_horizontal(&self, x: i32, y: i32) -> u32 {
        let mut word = 0;
        for ix in 0..4i32 {
            word |= (self.get(x + ix, y) as u32) << (ix * 8);
        }

        return word;
    }

    #[inline]
    fn get_vertical(&self, x: i32, y: i32) -> u32 {
        let mut word = 0;
        for iy in 0..4 {
            word |= (self.get(x, y + iy) as u32) << (iy * 8);
        }
        return word;
    }

    #[inline]
    fn get_diagonal_right(&self, x: i32, y: i32) -> u32 {
        let mut word = 0;
        for ixy in 0..4 {
            word |= (self.get(x + ixy, y + ixy) as u32) << (ixy * 8);
        }
        return word;
    }

    #[inline]
    fn get_diagonal_left(&self, x: i32, y: i32) -> u32 {
        let mut word = 0;
        for ixy in 0..4 {
            word |= (self.get(x - ixy, y + ixy) as u32) << (ixy * 8);
        }
        return word;
    }

    #[inline]
    fn get_x_diagonal_right(&self, x: i32, y: i32) -> u32 {
        let mut word = 0;
        for ixy in 0..3 {
            word |= (self.get(x - 1 + ixy, y - 1 + ixy) as u32) << (ixy * 8);
        }
        return word;
    }

    #[inline]
    fn get_x_diagonal_left(&self, x: i32, y: i32) -> u32 {
        let mut word = 0;
        for ixy in 0..3 {
            word |= (self.get(x + 1 - ixy, y - 1 + ixy) as u32) << (ixy * 8);
        }
        return word;
    }

    #[inline]
    fn get_x_diagonal_mask(&self, x: i32, y: i32) -> u32 {
        let lt = self.get(x - 1, y - 1);
        let rt = self.get(x + 1, y - 1);
        let lb = self.get(x - 1, y + 1);
        let rb = self.get(x + 1, y + 1);

        let mask = ((lt as u32 + rb as u32) << 0) | ((rt as u32 + lb as u32) << 8);
        return mask;
    }

    pub fn count_xmas(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let horizontal = self.get_horizontal(x, y);
                if horizontal == XMAS || horizontal == XMAS_REVERSE {
                    count += 1;
                }
                let vertical = self.get_vertical(x, y);
                if vertical == XMAS || vertical == XMAS_REVERSE {
                    count += 1;
                }
                let diagonal_right = self.get_diagonal_right(x, y);
                if diagonal_right == XMAS || diagonal_right == XMAS_REVERSE {
                    count += 1;
                }
                let diagonal_left = self.get_diagonal_left(x, y);
                if diagonal_left == XMAS || diagonal_left == XMAS_REVERSE {
                    count += 1;
                }
            }
        }
        return count;
    }

    pub fn count_xmas_check_xs(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x, y);
                if c != 'X' && c != 'S' {
                    continue;
                }
                let horizontal = self.get_horizontal(x, y);
                if horizontal == XMAS || horizontal == XMAS_REVERSE {
                    count += 1;
                }
                let vertical = self.get_vertical(x, y);
                if vertical == XMAS || vertical == XMAS_REVERSE {
                    count += 1;
                }
                let diagonal_right = self.get_diagonal_right(x, y);
                if diagonal_right == XMAS || diagonal_right == XMAS_REVERSE {
                    count += 1;
                }
                let diagonal_left = self.get_diagonal_left(x, y);
                if diagonal_left == XMAS || diagonal_left == XMAS_REVERSE {
                    count += 1;
                }
            }
        }
        return count;
    }

    pub fn count_mas_diagonal(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let diagonal_right = self.get_x_diagonal_right(x, y);
                let diagonal_left = self.get_x_diagonal_left(x, y);
                if (diagonal_right == MAS || diagonal_right == MAS_REVERSE) && (diagonal_left == MAS || diagonal_left == MAS_REVERSE) {
                    count += 1;
                }
            }
        }
        return count;
    }

    pub fn count_mas_diagonal_check_a(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x, y);
                if c != 'A' {
                    continue;
                }
                let diagonal_right = self.get_x_diagonal_right(x, y);
                let diagonal_left = self.get_x_diagonal_left(x, y);
                if (diagonal_right == MAS || diagonal_right == MAS_REVERSE) && (diagonal_left == MAS || diagonal_left == MAS_REVERSE) {
                    count += 1;
                }
            }
        }
        return count;
    }

    pub fn count_mas_diagonal_check_mask(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x, y);
                if c != 'A' {
                    continue;
                }
                if self.get_x_diagonal_mask(x, y) != MAS_MASK {
                    continue;
                }
                count += 1;
            }
        }
        return count;
    }

    pub fn count_mas_diagonal_check_if(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x, y);
                if c != 'A' {
                    continue;
                }

                let lt = self.get(x - 1, y - 1);
                let rt = self.get(x + 1, y - 1);
                let lb = self.get(x - 1, y + 1);
                let rb = self.get(x + 1, y + 1);

                if (lt != 'M' && lt != 'S') || (rt != 'M' && rt != 'S') || (lb != 'M' && lb != 'S') || (rb != 'M' && rb != 'S') {
                    continue;
                }
                if lt == rb || rt == lb {
                    continue;
                }

                count += 1;
            }
        }
        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    #[test]
    fn test_get_size() {
        let words = "XMAS\nXMAS\nXMAS\n".to_string();
        let ws = WordSearch::new(words);
        assert_eq!(ws.width, 4);
        assert_eq!(ws.height, 3);
    }

    #[test]
    fn test_get() {
        let words = "XMAS\nXMAS\nXMAS\nXMAS\nXMAS\n".to_string();
        let ws = WordSearch::new(words);
        assert_eq!(ws.get(0, 0), 'X');
        assert_eq!(ws.get(1, 1), 'M');
        assert_eq!(ws.get(2, 0), 'A');
        assert_eq!(ws.get(3, 3), 'S');
    }

    #[test]
    fn test_get_word() {
        let words = "XMASAMXMAS\nMMASAMXMAS\nAMASAMXMAS\nSMASAMXMAS\n".to_string();
        let ws = WordSearch::new(words);
        assert_eq!(ws.get_horizontal(0, 0), XMAS);
        assert_eq!(ws.get_horizontal(3, 2), XMAS_REVERSE);
        assert_eq!(ws.get_vertical(0, 0), XMAS);
        assert_eq!(ws.get_diagonal_right(0, 0), XMAS);
        assert_eq!(ws.get_diagonal_right(3, 0), XMAS_REVERSE);
        assert_eq!(ws.get_diagonal_left(9, 0), XMAS_REVERSE);
    }

    #[test]
    fn test_get_world_outside_bounds() {
        let words = "XMAS\nXMAS\nXMAS\nXMAS\n".to_string();
        let ws = WordSearch::new(words);
        assert_eq!(ws.get_horizontal(12, 5), 0);
        assert_eq!(ws.get_vertical(2, 8), 0);
        assert_eq!(ws.get_diagonal_right(12, 0), 0);
        assert_eq!(ws.get_diagonal_right(0, 7), 0);
    }

    #[test]
    fn test_count_xmas() {
        let words = "XMAS\nXMXX\nXXAX\nXXXS\nSAMX\n".to_string();
        let ws = WordSearch::new(words);
        assert_eq!(ws.count_xmas(), 3);
    }

    #[test]
    fn test_count_xmas_example() {
        let words = EXAMPLE_INPUT.to_string();

        let ws = WordSearch::new(words);
        assert_eq!(ws.count_xmas(), 18);
    }

    #[test]
    fn test_count_xmas_check_xs_example() {
        let words = EXAMPLE_INPUT.to_string();

        let ws = WordSearch::new(words);
        assert_eq!(ws.count_xmas_check_xs(), 18);
    }

    #[test]
    fn test_count_max_diagonal() {
        let words = "MAS\nMAS\nMXS\n".to_string();
        let ws = WordSearch::new(words);
        assert_eq!(ws.count_mas_diagonal(), 1);
    }

    #[test]
    fn test_count_mas_diagonal_example() {
        let words = EXAMPLE_INPUT.to_string();

        let ws = WordSearch::new(words);
        assert_eq!(ws.count_mas_diagonal(), 9);
    }

    #[test]
    fn test_count_mas_diagonal_check_a_example() {
        let words = EXAMPLE_INPUT.to_string();

        let ws = WordSearch::new(words);
        assert_eq!(ws.count_mas_diagonal_check_a(), 9);
    }

    #[test]
    fn test_count_mas_diagonal_check_mask_example() {
        let words = EXAMPLE_INPUT.to_string();

        let ws = WordSearch::new(words);
        assert_eq!(ws.count_mas_diagonal_check_mask(), 9);
    }

    #[test]
    fn test_count_mas_diagonal_check_if_example() {
        let words = EXAMPLE_INPUT.to_string();

        let ws = WordSearch::new(words);
        assert_eq!(ws.count_mas_diagonal_check_if(), 9);
    }

    #[test]
    fn test_count_mas_diagonal_example_dot() {
        let words = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
"#
        .to_string();
        let ws = WordSearch::new(words);
        assert_eq!(ws.count_mas_diagonal(), 9);
    }
}
