use std::io::BufRead;

pub fn parse_reports<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        if let Ok(line_str) = line {
            let numbers: Vec<i32> = line_str
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            reports.push(numbers);
        }
    }

    return reports;
}

fn check_diff(report: &Vec<i32>, first: usize, second: usize) -> bool {
    let diff = report[first] - report[second];
    return diff > 0 && diff <= 3;
}

pub fn is_safe_report(report: &Vec<i32>) -> bool {
    if report.len() == 0 {
        return false;
    }

    if report.len() == 1 {
        return true;
    }

    if report[0] < report[1] {
        for i in 1..report.len() {
            if !check_diff(report, i, i - 1) {
                return false;
            }
        }
    }
    else {
        for i in 1..report.len() {
            if !check_diff(report, i - 1, i) {
                return false;
            }
        }
    }
    return true;
}

pub fn is_safe_report_with_tolerance(report: &Vec<i32>) -> bool {
    if is_safe_report(report) {
        return true;
    }

    if report.len() == 2 {
        return true;
    }

    fn check_modified_report(report: &Vec<i32>, index: usize) -> bool {
        let mut modified_report = report.clone();
        modified_report.remove(index);
        return is_safe_report(&modified_report)
    }
    fn check_all_variants_of_modification(report: &Vec<i32>, first: usize, second: usize) -> bool {
        // Check if the report is safe after removing the first element.
        if check_modified_report(report, first) {
            return true;
        }
        // Check if the report is safe after removing the second element.
        if check_modified_report(report, second) {
            return true;
        }

        // Check if the report is safe after removing the 0 index.
        // We check this only if it is the second pair and we have a negative diff.
        // It may happen that after removing the first element the report is now increasing instead of decreasing.
        if std::cmp::min(first, second) == 1 {
            let diff = report[first] - report[second];
            if diff < 0 {
                if check_modified_report(report, 0) {
                    return true;
                }
            }
        }

        return false;
    }

    if report[0] < report[1] {
        for i in 1..report.len() {
            if !check_diff(report, i, i - 1) {
                return check_all_variants_of_modification(report, i, i - 1);
            }
        }
    }
    else {
        for i in 1..report.len() {
            if !check_diff(report, i - 1, i) {
                return check_all_variants_of_modification(report, i - 1, i);
            }
        }
    }
    return true;
}

pub fn count_safe_reports(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|r| is_safe_report(r)).count()
}

pub fn count_safe_reports_with_tolerance(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|r| is_safe_report_with_tolerance(r)).count()
}

// Test the function
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_reports() {
        let input = "1 2 3\n4 5 6\n7 8 9";
        let reader = Cursor::new(input);
        let reports = parse_reports(reader);
        assert_eq!(reports.len(), 3);
        assert_eq!(reports[0], vec![1, 2, 3]);
        assert_eq!(reports[1], vec![4, 5, 6]);
        assert_eq!(reports[2], vec![7, 8, 9]);
    }

    #[test]
    fn test_is_safe_report() {
        assert_eq!(is_safe_report(&vec![1]), true);
        assert_eq!(is_safe_report(&vec![1, 2]), true);
        assert_eq!(is_safe_report(&vec![1, 3, 6]), true);
        assert_eq!(is_safe_report(&vec![2, 1]), true);
        assert_eq!(is_safe_report(&vec![6, 4, 1]), true);

        assert_eq!(is_safe_report(&vec![]), false);
        assert_eq!(is_safe_report(&vec![1, 5]), false);
        assert_eq!(is_safe_report(&vec![1, 3, 2]), false);
        assert_eq!(is_safe_report(&vec![10, 9, 1]), false);
    }

    #[test]
    fn test_is_safe_report_example() {
        assert_eq!(is_safe_report(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe_report(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe_report(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe_report(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_safe_report(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_safe_report(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_is_safe_report_with_tolerance_example() {
        assert_eq!(is_safe_report_with_tolerance(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe_report_with_tolerance(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe_report_with_tolerance(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe_report_with_tolerance(&vec![1, 3, 2, 4, 5]), true);
        assert_eq!(is_safe_report_with_tolerance(&vec![8, 6, 4, 4, 1]), true);
        assert_eq!(is_safe_report_with_tolerance(&vec![1, 3, 6, 7, 9]), true);

        // Extra modifications
        assert_eq!(is_safe_report_with_tolerance(&vec![1, 3, 1, 4, 5]), true); // Safe by removing the third level, 1.
        assert_eq!(is_safe_report_with_tolerance(&vec![3, 6, 4, 2, 1]), true); // Safe by removing the first level, 5.
    }
}
