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

pub fn is_safe_report(report: &Vec<i32>) -> bool {
    if report.len() == 0 {
        return false;
    }

    if report.len() == 1 {
        return true;
    }

    if report[0] < report[1] {
        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];
            if diff <= 0 || diff > 3 {
                return false;
            }
        }
    }
    else {
        for i in 1..report.len() {
            let diff = report[i - 1] - report[i];
            if diff <= 0 || diff > 3 {
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

    if report.len() < 2 {
        return false;
    }

    if report[0] < report[1] {
        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];
            if diff <= 0 || diff > 3 {
                // Remove the element at index i and check if the report is safe
                let mut modified_report = report.clone();
                modified_report.remove(i);
                if is_safe_report(&modified_report) {
                    return true;
                }
                // Remove the element at index i - 1 and check if the report is safe
                modified_report = report.clone();
                modified_report.remove(i - 1);
                if is_safe_report(&modified_report) {
                    return true;
                }
                // Remove the element at index 0 if checking between the index 2
                // and the diff is negative
                if i == 2 && diff < 0 {
                    modified_report = report.clone();
                    modified_report.remove(0);
                    if is_safe_report(&modified_report) {
                        return true;
                    }
                }

                // Check only the first problem
                return false;
            }
        }
    }
    else {
        for i in 1..report.len() {
            let diff = report[i - 1] - report[i];
            if diff <= 0 || diff > 3 {
                // Remove the element at index i and check if the report is safe
                let mut modified_report = report.clone();
                modified_report.remove(i);
                if is_safe_report(&modified_report) {
                    return true;
                }
                // Remove the element at index i - 1 and check if the report is safe
                modified_report = report.clone();
                modified_report.remove(i - 1);
                if is_safe_report(&modified_report) {
                    return true;
                }
                // Remove the element at index 0 if checking between the index 2
                // and the diff is negative
                if i == 2 && diff < 0 {
                    modified_report = report.clone();
                    modified_report.remove(0);
                    if is_safe_report(&modified_report) {
                        return true;
                    }
                }

                // Check only the first problem
                return false;
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
