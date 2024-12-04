const XMAS: u32 = (('X' as u32) << 0) | (('M' as u32) << 8) | (('A' as u32) << 16) | (('S' as u32)  << 24);
const XMAS_REVERSE: u32 = (('S' as u32) << 0) | (('A' as u32) << 8) | (('M' as u32) << 16) | (('X' as u32)  << 24);

pub struct WordSearch {
    words: String,
    width: usize,
    height: usize,
}

impl WordSearch {
    pub fn new(words: String) -> WordSearch {
        let width = words.find('\n').unwrap();
        let height = words.len() / (width + 1);
        WordSearch { words, width, height }
    }

    fn get(&self, x: usize, y: usize) -> char {
        if y >= self.height || x >= self.width {
            return 0 as char;
        }

        return self.words.chars().nth(y * (self.width + 1) + x).unwrap()
    }

    fn get_horizontal(&self, x: usize, y: usize) -> u32 {
        let mut word = 0;
        for ix in 0..4 {
            word |= (self.get(x + ix, y) as u32) << (ix * 8);
        }

        return word;
    }

    fn get_vertical(&self, x: usize, y: usize) -> u32 {
        let mut word = 0;
        for iy in 0..4 {
            word |= (self.get(x, y + iy) as u32) << (iy * 8);
        }
        return word;
    }

    fn get_diagonal_right(&self, x: usize, y: usize) -> u32 {
        let mut word = 0;
        for ixy in 0..4 {
            word |= (self.get(x + ixy, y + ixy) as u32) << (ixy * 8);
        }
        return word;
    }

    fn get_diagonal_left(&self, x: usize, y: usize) -> u32 {
        let mut word = 0;
        for ixy in 0..4 {
            word |= (self.get(x - ixy, y + ixy) as u32) << (ixy * 8);
        }
        return word;
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let words = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#.to_string();
        let ws = WordSearch::new(words);
        assert_eq!(ws.count_xmas(), 18);
    }
}
