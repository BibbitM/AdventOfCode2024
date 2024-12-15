use crate::char_map::CharMap;
use crate::matrix::Matrix;

const TOP: (i32, i32) = (0, -1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);
const BOTTOM: (i32, i32) = (0, 1);

const DIRECTIONS: [(i32, i32); 4] = [TOP, LEFT, RIGHT, BOTTOM];

struct Fence {
    area: i32,
    perimeter: i32,
}

pub fn calculate_fencing_price(garden: &CharMap) -> i32 {
    let mut garden_regions = Matrix::new(garden.width as usize, garden.height as usize, 0u32);
    let mut region_fences = Vec::<Fence>::new();
    let mut region_parents = Vec::<u32>::new();

    for y in 0..garden.height {
        for x in 0..garden.width {
            let plot = garden.get(x, y);
            let top_pos = (x + TOP.0, y + TOP.1);
            let top = garden.get(top_pos.0, top_pos.1);
            let left_pos = (x + LEFT.0, y + LEFT.1);
            let left = garden.get(left_pos.0, left_pos.1);

            let mut perimeter = 0;
            for d in DIRECTIONS.iter() {
                if garden.get(x + d.0, y + d.1) != plot {
                    perimeter += 1;
                }
            }

            if plot != top && plot != left {
                let region = region_fences.len() as u32;
                region_fences.push(Fence { area: 1, perimeter });
                region_parents.push(u32::max_value());
                garden_regions.set(x as usize, y as usize, region);
            } else if plot == top && plot == left {
                let mut top_region = *garden_regions.get(top_pos.0 as usize, top_pos.1 as usize);
                let mut left_region = *garden_regions.get(left_pos.0 as usize, left_pos.1 as usize);
                while region_parents[top_region as usize] != u32::max_value() {
                    top_region = region_parents[top_region as usize];
                }
                while region_parents[left_region as usize] != u32::max_value() {
                    left_region = region_parents[left_region as usize];
                }

                if top_region < left_region {
                    // Top region is the parent of left region.
                    region_parents[left_region as usize] = top_region;
                    // Add data from left fence to right fence.
                    region_fences[top_region as usize].area += region_fences[left_region as usize].area;
                    region_fences[top_region as usize].perimeter += region_fences[left_region as usize].perimeter;
                    // Clear left fence data.
                    region_fences[left_region as usize].area = 0;
                    region_fences[left_region as usize].perimeter = 0;
                } else if left_region < top_region {
                    // Left region is the parent of top region.
                    region_parents[top_region as usize] = left_region;
                    // Add data from top fence to left fence.
                    region_fences[left_region as usize].area += region_fences[top_region as usize].area;
                    region_fences[left_region as usize].perimeter += region_fences[top_region as usize].perimeter;
                    // Clear top fence data.
                    region_fences[top_region as usize].area = 0;
                    region_fences[top_region as usize].perimeter = 0;
                }

                let region = std::cmp::min(top_region, left_region);
                region_fences[region as usize].area += 1;
                region_fences[region as usize].perimeter += perimeter;
                garden_regions.set(x as usize, y as usize, region);
            } else if plot == top {
                let mut top_region = *garden_regions.get(top_pos.0 as usize, top_pos.1 as usize);
                while region_parents[top_region as usize] != u32::max_value() {
                    top_region = region_parents[top_region as usize];
                }
                region_fences[top_region as usize].area += 1;
                region_fences[top_region as usize].perimeter += perimeter;
                garden_regions.set(x as usize, y as usize, top_region);
            } else { //< plot == left
                let mut left_region = *garden_regions.get(left_pos.0 as usize, left_pos.1 as usize);
                while region_parents[left_region as usize] != u32::max_value() {
                    left_region = region_parents[left_region as usize];
                }
                region_fences[left_region as usize].area += 1;
                region_fences[left_region as usize].perimeter += perimeter;
                garden_regions.set(x as usize, y as usize, left_region);
            }
        }
    }

    let mut price = 0;

    for fence in region_fences.iter() {
        price += fence.area * fence.perimeter;
    }

    return price;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"AAAA
BBCD
BBCC
EEEC
"#;

    const ANOTHER_INPUT: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"#;

    const EXAMPLE_INPUT: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

    #[test]
    fn test_calculate_fencing_price_simple() {
        let garden = CharMap::new(SIMPLE_INPUT.to_string());
        assert_eq!(calculate_fencing_price(&garden), 140);
    }

    #[test]
    fn test_calculate_fencing_price_another() {
        let garden = CharMap::new(ANOTHER_INPUT.to_string());
        assert_eq!(calculate_fencing_price(&garden), 772);
    }

    #[test]
    fn test_calculate_fencing_price_example() {
        let garden = CharMap::new(EXAMPLE_INPUT.to_string());
        assert_eq!(calculate_fencing_price(&garden), 1930);
    }
}
