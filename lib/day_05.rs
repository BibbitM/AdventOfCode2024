use std::io::BufRead;

pub fn parse_pages<R: BufRead>(reader: R) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut page_ordering_rules: Vec<(i32, i32)> = Vec::new();
    let mut pages_to_produce: Vec<Vec<i32>> = Vec::new();

    let mut parse_rules = true;

    for line in reader.lines() {
        if let Ok(line_content) = line {
            if line_content.is_empty() {
                parse_rules = false;
            } else if parse_rules {
                let parts: Vec<&str> = line_content.split('|').collect();
                if parts.len() == 2 {
                    if let (Ok(first), Ok(second)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        page_ordering_rules.push((first, second));
                    }
                }
            } else {
                let parse_pages: Vec<i32> = line_content.split(',').filter_map(|s| s.parse::<i32>().ok()).collect();
                pages_to_produce.push(parse_pages);
            }
        }
    }

    page_ordering_rules.sort();

    (page_ordering_rules, pages_to_produce)
}

fn check_page_rules(page_ordering_rules: &Vec<(i32, i32)>, page: &Vec<i32>) -> bool {
    for i in 0..page.len() - 1 {
        for j in i + 1..page.len() {
            let page_pair = (page[j], page[i]);
            if page_ordering_rules.contains(&page_pair) {
                return false;
            }
        }
    }

    return true;
}

fn get_middle_page(page: Vec<i32>) -> i32 {
    let middle_page_idx = page.len() / 2;
    return page[middle_page_idx];
}

pub fn sum_of_correct_middle_pages(page_ordering_rules: &Vec<(i32, i32)>, pages_to_produce: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for page in pages_to_produce {
        if check_page_rules(page_ordering_rules, page) {
            sum += get_middle_page(page.to_vec());
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const EXAMPLE_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
    #[test]
    fn test_parse_pages() {
        let data = "56|78\n12|34\n\n12,34,56\n78,56,12\n";
        let cursor = Cursor::new(data);
        let (page_ordering_rules, pages_to_produce) = parse_pages(cursor);

        assert_eq!(page_ordering_rules, vec![(12, 34), (56, 78)]);
        assert_eq!(pages_to_produce, vec![vec![12, 34, 56], vec![78, 56, 12]]);
    }

    #[test]
    fn test_check_page_rules() {
        let cursor = Cursor::new(EXAMPLE_INPUT);
        let (page_ordering_rules, pages_to_produce) = parse_pages(cursor);

        assert_eq!(check_page_rules(&page_ordering_rules, &pages_to_produce[0]), true);
        assert_eq!(check_page_rules(&page_ordering_rules, &pages_to_produce[1]), true);
        assert_eq!(check_page_rules(&page_ordering_rules, &pages_to_produce[2]), true);
        assert_eq!(check_page_rules(&page_ordering_rules, &pages_to_produce[3]), false);
        assert_eq!(check_page_rules(&page_ordering_rules, &pages_to_produce[4]), false);
        assert_eq!(check_page_rules(&page_ordering_rules, &pages_to_produce[5]), false);
    }

    #[test]
    fn test_get_middle_page() {
        assert_eq!(get_middle_page(vec![75, 47, 61, 53, 29]), 61);
        assert_eq!(get_middle_page(vec![97, 61, 53, 29, 13]), 53);
        assert_eq!(get_middle_page(vec![75, 29, 13]), 29);
    }

    #[test]
    fn test_get_sum_of_correct_middle_pages_example() {
        let cursor = Cursor::new(EXAMPLE_INPUT);
        let (page_ordering_rules, pages_to_produce) = parse_pages(cursor);

        assert_eq!(sum_of_correct_middle_pages(&page_ordering_rules, &pages_to_produce), 143);
    }
}
