use crate::char_map::CharMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(i32, i32);

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
fn find_tailhead_score(map: &CharMap, start: Pos) -> usize {
    assert_eq!(map.get(start.0, start.1), '0');

    let mut points_to_visit = vec![start];
    let mut char_to_check = '1';
    let last_char = ('9' as u8 + 1) as char;
    while !points_to_visit.is_empty() && char_to_check != last_char {
        let mut next_points = Vec::new();
        for point in points_to_visit {
            for dir in DIRECTIONS.iter() {
                let next = Pos(point.0 + dir.0, point.1 + dir.1);
                if map.get(next.0, next.1) == char_to_check {
                    next_points.push(next);
                }
            }
        }

        points_to_visit = next_points;
        points_to_visit.sort();
        points_to_visit.dedup();

        char_to_check = (char_to_check as u8 + 1) as char;
    }

    if char_to_check == last_char {
        return points_to_visit.len();
    } else {
        return 0;
    }
}

pub fn calculate_sum_of_tailhead_scores(map: &CharMap) -> usize {
    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == '0' {
                sum += find_tailhead_score(map, Pos(x, y));
            }
        }
    }
    return sum;
}

fn find_tailhead_rating(map: &CharMap, start: Pos) -> usize {
    assert_eq!(map.get(start.0, start.1), '0');

    return find_tailhead_rating_recursive(&map, start, '1');
}

fn find_tailhead_rating_recursive(map: &CharMap, point: Pos, char_to_check: char) -> usize {
    let mut rating = 0;

    let next_char_to_check = (char_to_check as u8 + 1) as char;
    for dir in DIRECTIONS.iter() {
        let next_point = Pos(point.0 + dir.0, point.1 + dir.1);
        if map.get(next_point.0, next_point.1) == char_to_check {
            if char_to_check == '9' {
                rating += 1;
            } else {
                rating += find_tailhead_rating_recursive(map, next_point, next_char_to_check);
            }
        }
    }

    return rating;
}

pub fn calculate_sum_of_tailhead_ratings(map: &CharMap) -> usize {
    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == '0' {
                sum += find_tailhead_rating(map, Pos(x, y));
            }
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn test_find_tailhead_score() {
        let input = r#"0123
1234
8765
9876
"#;
        let map = CharMap::new(input.to_string());

        assert_eq!(find_tailhead_score(&map, Pos(0, 0)), 1);
    }

    #[test]
    fn test_calculate_sum_of_tailhead_scores_example() {
        let map = CharMap::new(EXAMPLE_INPUT.to_string());
        assert_eq!(calculate_sum_of_tailhead_scores(&map), 36);
    }

    #[test]
    fn test_find_tailhead_rating() {
        let map = CharMap::new(EXAMPLE_INPUT.to_string());
        assert_eq!(find_tailhead_rating(&map, Pos(2, 0)), 20);
        assert_eq!(find_tailhead_rating(&map, Pos(4, 0)), 24);
    }

    #[test]
    fn test_calculate_sum_of_tailhead_ratings_example() {
        let map = CharMap::new(EXAMPLE_INPUT.to_string());
        assert_eq!(calculate_sum_of_tailhead_ratings(&map), 81);
    }
}
