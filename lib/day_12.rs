use crate::char_map::CharMap;
use crate::matrix::Matrix;

const TOP: (i32, i32) = (0, -1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);
const BOTTOM: (i32, i32) = (0, 1);

const DIRECTIONS: [(i32, i32); 4] = [TOP, LEFT, BOTTOM, RIGHT];

struct Fence {
    area: i32,
    perimeter: i32,
    corners: i32,
}

pub fn calculate_fencing_price(garden: &CharMap) -> (i32, i32) {
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
            let mut corners = 0;
            for i in 0..4 {
                let d1 = DIRECTIONS[i];
                let d2 = DIRECTIONS[(i + 1) % 4];
                if garden.get(x + d1.0, y + d1.1) != plot {
                    perimeter += 1;
                }
                if garden.get(x + d1.0, y + d1.1) != plot && garden.get(x + d2.0, y + d2.1) != plot {
                    corners += 1;
                } else if garden.get(x + d1.0, y + d1.1) == plot && garden.get(x + d2.0, y + d2.1) == plot && garden.get(x + d1.0 + d2.0, y + d1.1 + d2.1) != plot {
                    corners += 1;
                }
            }

            if plot != top && plot != left {
                let region = region_fences.len() as u32;
                region_fences.push(Fence { area: 1, perimeter, corners });
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
                    region_fences[top_region as usize].corners += region_fences[left_region as usize].corners;
                    // Clear left fence data.
                    region_fences[left_region as usize].area = 0;
                    region_fences[left_region as usize].perimeter = 0;
                    region_fences[left_region as usize].corners = 0;
                } else if left_region < top_region {
                    // Left region is the parent of top region.
                    region_parents[top_region as usize] = left_region;
                    // Add data from top fence to left fence.
                    region_fences[left_region as usize].area += region_fences[top_region as usize].area;
                    region_fences[left_region as usize].perimeter += region_fences[top_region as usize].perimeter;
                    region_fences[left_region as usize].corners += region_fences[top_region as usize].corners;
                    // Clear top fence data.
                    region_fences[top_region as usize].area = 0;
                    region_fences[top_region as usize].perimeter = 0;
                    region_fences[top_region as usize].corners = 0;
                }

                let region = std::cmp::min(top_region, left_region);
                region_fences[region as usize].area += 1;
                region_fences[region as usize].perimeter += perimeter;
                region_fences[region as usize].corners += corners;
                garden_regions.set(x as usize, y as usize, region);
            } else if plot == top {
                let mut top_region = *garden_regions.get(top_pos.0 as usize, top_pos.1 as usize);
                while region_parents[top_region as usize] != u32::max_value() {
                    top_region = region_parents[top_region as usize];
                }
                region_fences[top_region as usize].area += 1;
                region_fences[top_region as usize].perimeter += perimeter;
                region_fences[top_region as usize].corners += corners;
                garden_regions.set(x as usize, y as usize, top_region);
            } else {
                //< plot == left
                let mut left_region = *garden_regions.get(left_pos.0 as usize, left_pos.1 as usize);
                while region_parents[left_region as usize] != u32::max_value() {
                    left_region = region_parents[left_region as usize];
                }
                region_fences[left_region as usize].area += 1;
                region_fences[left_region as usize].perimeter += perimeter;
                region_fences[left_region as usize].corners += corners;
                garden_regions.set(x as usize, y as usize, left_region);
            }
        }
    }

    let mut price_v1 = 0;
    let mut prive_v2 = 0;

    for fence in region_fences.iter() {
        price_v1 += fence.area * fence.perimeter;
        prive_v2 += fence.area * fence.corners;
    }

    return (price_v1, prive_v2);
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
        let garden = CharMap::new(&SIMPLE_INPUT.to_string());
        assert_eq!(calculate_fencing_price(&garden).0, 140);
    }

    #[test]
    fn test_calculate_fencing_price_another() {
        let garden = CharMap::new(&ANOTHER_INPUT.to_string());
        assert_eq!(calculate_fencing_price(&garden).0, 772);
    }

    #[test]
    fn test_calculate_fencing_price_example() {
        let garden = CharMap::new(&EXAMPLE_INPUT.to_string());
        let (price_v1, price_v2) = calculate_fencing_price(&garden);
        assert_eq!(price_v1, 1930);
        assert_eq!(price_v2, 1206);
    }
}
