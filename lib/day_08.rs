use crate::char_map::CharMap;
use std::collections::HashSet;

pub fn gather_antennas(antennas_map: &CharMap) -> std::collections::HashMap<char, Vec<(i32, i32)>> {
    let mut antennas: std::collections::HashMap<char, Vec<(i32, i32)>> = std::collections::HashMap::new();

    for y in 0..antennas_map.height {
        for x in 0..antennas_map.width {
            let pos = antennas_map.get(x, y);
            if pos != '.' {
                let antenna = antennas.entry(pos).or_insert(Vec::new());
                antenna.push((x, y));
            }
        }
    }

    return antennas;
}

pub fn find_antinodes(antennas: &std::collections::HashMap<char, Vec<(i32, i32)>>, antennas_map: &CharMap) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();

    #[inline]
    fn is_in_bounds(pos: &(i32, i32), antennas_map: &CharMap) -> bool {
        return pos.0 >= 0 && pos.0 < antennas_map.width && pos.1 >= 0 && pos.1 < antennas_map.height;
    }

    for antenna in antennas.values() {
        for i in 0..antenna.len() - 1 {
            for j in i + 1..antenna.len() {
                let antenna1 = antenna[i];
                let antenna2 = antenna[j];

                let offset = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);

                let antinode1 = (antenna1.0 - offset.0, antenna1.1 - offset.1);
                let antinode2 = (antenna2.0 + offset.0, antenna2.1 + offset.1);

                if is_in_bounds(&antinode1, &antennas_map) && !antinodes.contains(&antinode1) {
                    antinodes.push(antinode1);
                }
                if is_in_bounds(&antinode2, &antennas_map) && !antinodes.contains(&antinode2) {
                    antinodes.push(antinode2);
                }
            }
        }
    }

    return antinodes;
}

pub fn find_antinodes_sort_dedup(antennas: &std::collections::HashMap<char, Vec<(i32, i32)>>, antennas_map: &CharMap) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();

    #[inline]
    fn is_in_bounds(pos: &(i32, i32), antennas_map: &CharMap) -> bool {
        return pos.0 >= 0 && pos.0 < antennas_map.width && pos.1 >= 0 && pos.1 < antennas_map.height;
    }

    for antenna in antennas.values() {
        for i in 0..antenna.len() - 1 {
            for j in i + 1..antenna.len() {
                let antenna1 = antenna[i];
                let antenna2 = antenna[j];

                let offset = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);

                let antinode1 = (antenna1.0 - offset.0, antenna1.1 - offset.1);
                let antinode2 = (antenna2.0 + offset.0, antenna2.1 + offset.1);

                if is_in_bounds(&antinode1, &antennas_map) {
                    antinodes.push(antinode1);
                }
                if is_in_bounds(&antinode2, &antennas_map) {
                    antinodes.push(antinode2);
                }
            }
        }
    }

    antinodes.sort();
    antinodes.dedup();
    return antinodes;
}

pub fn find_antinodes_hash_set(antennas: &std::collections::HashMap<char, Vec<(i32, i32)>>, antennas_map: &CharMap) -> Vec<(i32, i32)> {
    let mut antinodes = HashSet::new();

    #[inline]
    fn is_in_bounds(pos: &(i32, i32), antennas_map: &CharMap) -> bool {
        return pos.0 >= 0 && pos.0 < antennas_map.width && pos.1 >= 0 && pos.1 < antennas_map.height;
    }

    for antenna in antennas.values() {
        for i in 0..antenna.len() - 1 {
            for j in i + 1..antenna.len() {
                let antenna1 = antenna[i];
                let antenna2 = antenna[j];

                let offset = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);

                let antinode1 = (antenna1.0 - offset.0, antenna1.1 - offset.1);
                let antinode2 = (antenna2.0 + offset.0, antenna2.1 + offset.1);

                if is_in_bounds(&antinode1, &antennas_map) {
                    antinodes.insert(antinode1);
                }
                if is_in_bounds(&antinode2, &antennas_map) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    return antinodes.iter().copied().collect();
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a.abs();
}

pub fn find_antinodes_in_line(antennas: &std::collections::HashMap<char, Vec<(i32, i32)>>, antennas_map: &CharMap) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();

    #[inline]
    fn is_in_bounds(pos: &(i32, i32), antennas_map: &CharMap) -> bool {
        return pos.0 >= 0 && pos.0 < antennas_map.width && pos.1 >= 0 && pos.1 < antennas_map.height;
    }

    for antenna in antennas.values() {
        for i in 0..antenna.len() - 1 {
            for j in i + 1..antenna.len() {
                let antenna1 = antenna[i];
                let antenna2 = antenna[j];

                if !antinodes.contains(&antenna1) {
                    antinodes.push(antenna1);
                }
                if !antinodes.contains(&antenna2) {
                    antinodes.push(antenna2);
                }

                let offset = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);
                let count = gcd(offset.0, offset.1);
                let offset = (offset.0 / count, offset.1 / count);

                for n in 1..count {
                    let antinode = (antenna1.0 + offset.0 * n, antenna1.1 + offset.1 * n);
                    if !antinodes.contains(&antinode) {
                        antinodes.push(antinode);
                    }
                }

                let mut n = 1;
                loop {
                    let antinode = (antenna1.0 - offset.0 * n, antenna1.1 - offset.1 * n);
                    if !is_in_bounds(&antinode, &antennas_map) {
                        break;
                    }
                    if !antinodes.contains(&antinode) {
                        antinodes.push(antinode);
                    }
                    n += 1;
                }

                let mut n = 1;
                loop {
                    let antinode = (antenna2.0 + offset.0 * n, antenna2.1 + offset.1 * n);
                    if !is_in_bounds(&antinode, &antennas_map) {
                        break;
                    }
                    if !antinodes.contains(&antinode) {
                        antinodes.push(antinode);
                    }
                    n += 1;
                }
            }
        }
    }

    return antinodes;
}

pub fn find_antinodes_in_line_sort_dedup(antennas: &std::collections::HashMap<char, Vec<(i32, i32)>>, antennas_map: &CharMap) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();

    #[inline]
    fn is_in_bounds(pos: &(i32, i32), antennas_map: &CharMap) -> bool {
        return pos.0 >= 0 && pos.0 < antennas_map.width && pos.1 >= 0 && pos.1 < antennas_map.height;
    }

    for antenna in antennas.values() {
        for i in 0..antenna.len() - 1 {
            for j in i + 1..antenna.len() {
                let antenna1 = antenna[i];
                let antenna2 = antenna[j];

                antinodes.push(antenna1);
                antinodes.push(antenna2);

                let offset = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);
                let count = gcd(offset.0, offset.1);
                let offset = (offset.0 / count, offset.1 / count);

                for n in 1..count {
                    let antinode = (antenna1.0 + offset.0 * n, antenna1.1 + offset.1 * n);
                    antinodes.push(antinode);
                }

                let mut n = 1;
                loop {
                    let antinode = (antenna1.0 - offset.0 * n, antenna1.1 - offset.1 * n);
                    if !is_in_bounds(&antinode, &antennas_map) {
                        break;
                    }
                    antinodes.push(antinode);
                    n += 1;
                }

                let mut n = 1;
                loop {
                    let antinode = (antenna2.0 + offset.0 * n, antenna2.1 + offset.1 * n);
                    if !is_in_bounds(&antinode, &antennas_map) {
                        break;
                    }
                    antinodes.push(antinode);
                    n += 1;
                }
            }
        }
    }

    antinodes.sort();
    antinodes.dedup();
    return antinodes;
}

pub fn find_antinodes_in_line_hash_set(antennas: &std::collections::HashMap<char, Vec<(i32, i32)>>, antennas_map: &CharMap) -> Vec<(i32, i32)> {
    let mut antinodes = HashSet::new();

    #[inline]
    fn is_in_bounds(pos: &(i32, i32), antennas_map: &CharMap) -> bool {
        return pos.0 >= 0 && pos.0 < antennas_map.width && pos.1 >= 0 && pos.1 < antennas_map.height;
    }

    for antenna in antennas.values() {
        for i in 0..antenna.len() - 1 {
            for j in i + 1..antenna.len() {
                let antenna1 = antenna[i];
                let antenna2 = antenna[j];

                antinodes.insert(antenna1);
                antinodes.insert(antenna2);

                let offset = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);
                let count = gcd(offset.0, offset.1);
                let offset = (offset.0 / count, offset.1 / count);

                for n in 1..count {
                    let antinode = (antenna1.0 + offset.0 * n, antenna1.1 + offset.1 * n);
                    antinodes.insert(antinode);
                }

                let mut n = 1;
                loop {
                    let antinode = (antenna1.0 - offset.0 * n, antenna1.1 - offset.1 * n);
                    if !is_in_bounds(&antinode, &antennas_map) {
                        break;
                    }
                    antinodes.insert(antinode);
                    n += 1;
                }

                let mut n = 1;
                loop {
                    let antinode = (antenna2.0 + offset.0 * n, antenna2.1 + offset.1 * n);
                    if !is_in_bounds(&antinode, &antennas_map) {
                        break;
                    }
                    antinodes.insert(antinode);
                    n += 1;
                }
            }
        }
    }

    return antinodes.iter().copied().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    #[test]
    fn test_find_start_position() {
        let antennas_map = CharMap::new(&EXAMPLE_INPUT.to_string());

        let antennas = gather_antennas(&antennas_map);

        assert_eq!(antennas.len(), 2);
        assert_eq!(antennas[&'0'].len(), 4);
        assert_eq!(antennas[&'A'].len(), 3);
    }

    #[test]
    fn test_find_antinodes() {
        let antennas_map = CharMap::new(&EXAMPLE_INPUT.to_string());
        let antennas = gather_antennas(&antennas_map);
        let antinodes = find_antinodes(&antennas, &antennas_map);

        assert_eq!(antinodes.len(), 14);
    }

    #[test]
    fn test_find_antinodes_sort_dedup() {
        let antennas_map = CharMap::new(&EXAMPLE_INPUT.to_string());
        let antennas = gather_antennas(&antennas_map);
        let antinodes = find_antinodes_sort_dedup(&antennas, &antennas_map);

        assert_eq!(antinodes.len(), 14);
    }

    #[test]
    fn test_find_antinodes_hash_set() {
        let antennas_map = CharMap::new(&EXAMPLE_INPUT.to_string());
        let antennas = gather_antennas(&antennas_map);
        let antinodes = find_antinodes_hash_set(&antennas, &antennas_map);

        assert_eq!(antinodes.len(), 14);
    }

    #[test]
    fn test_find_antinodes_in_line() {
        let antennas_map = CharMap::new(&EXAMPLE_INPUT.to_string());
        let antennas = gather_antennas(&antennas_map);
        let antinodes = find_antinodes_in_line(&antennas, &antennas_map);

        assert_eq!(antinodes.len(), 34);
    }

    #[test]
    fn test_find_antinodes_in_line_sort_dedup() {
        let antennas_map = CharMap::new(&EXAMPLE_INPUT.to_string());
        let antennas = gather_antennas(&antennas_map);
        let antinodes = find_antinodes_in_line_sort_dedup(&antennas, &antennas_map);

        assert_eq!(antinodes.len(), 34);
    }

    #[test]
    fn test_find_antinodes_in_line_hash_set() {
        let antennas_map = CharMap::new(&EXAMPLE_INPUT.to_string());
        let antennas = gather_antennas(&antennas_map);
        let antinodes = find_antinodes_in_line_hash_set(&antennas, &antennas_map);

        assert_eq!(antinodes.len(), 34);
    }
}
