use crate::inputs;

pub fn run() -> String {
    let reports = inputs::parse_input_for_day(2, parse_input);
    let num_safe_reports = count_safe_reports(reports);

    format!("There are {num_safe_reports} safe reports")
}

fn count_safe_reports(reports: Vec<Report>) -> usize {
    reports
        .iter()
        .filter(|report| {
            let all_increasing = report
                .levels
                .windows(2)
                .map(|x| x[0].saturating_sub(x[1]))
                .all(|diff| (1..4).contains(&diff));

            let all_decreasing = report
                .levels
                .windows(2)
                .map(|x| x[1].saturating_sub(x[0]))
                .all(|diff| (1..4).contains(&diff));

            all_increasing || all_decreasing
        })
        .count()
}

fn parse_input(input: String) -> Vec<Report> {
    let mut reports = Vec::new();
    for line in input.lines() {
        let mut levels = Vec::new();
        for s in line.split_whitespace() {
            let i: u32 = s.trim().parse().expect("Error while parsing input");
            levels.push(i);
        }
        reports.push(Report { levels });
    }

    reports
}

struct Report {
    levels: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn parse_input_test() {
        let reports = parse_input(String::from(TEST_INPUT));
        assert_eq!(reports.len(), 6);
        reports.iter().for_each(|r| {
            assert_eq!(r.levels.len(), 5);
        });
    }

    #[test]
    fn num_safe_reports() {
        let reports = parse_input(String::from(TEST_INPUT));
        assert_eq!(2, count_safe_reports(reports));
    }
}
