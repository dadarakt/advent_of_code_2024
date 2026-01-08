use crate::inputs;

pub fn run() -> String {
    let reports = inputs::parse_input_for_day(2, parse_input);
    let num_safe_reports = count_safe_reports(&reports);
    let num_safe_reports_dampened = count_safe_reports_with_dampener(&reports);

    format!(
        "There are {num_safe_reports} safe reports and {num_safe_reports_dampened} dampened safe reports"
    )
}

fn count_safe_reports(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|report| Report::is_safe(report))
        .count()
}

fn count_safe_reports_with_dampener(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|report| {
            let p = Report::is_safe_dampened(report);
            if p {
                dbg!(&report.levels);
            }
            p
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

impl Report {
    pub fn is_safe(r: &Report) -> bool {
        let all_increasing = r
            .levels
            .windows(2)
            .map(|x| x[0].saturating_sub(x[1]))
            .all(|diff| (1..4).contains(&diff));

        let all_decreasing = r
            .levels
            .windows(2)
            .map(|x| x[1].saturating_sub(x[0]))
            .all(|diff| (1..4).contains(&diff));

        all_increasing || all_decreasing
    }

    pub fn is_safe_dampened(r: &Report) -> bool {
        Report::is_safe(r) || Report::is_safe_with_skip(&r.levels)
    }

    fn is_safe_with_skip(v: &[u32]) -> bool {
        v.len() < 3
            || (0..v.len()).any(|skip| {
                let mut iter = SkipOneIter::build(v.iter(), skip);
                let first = iter.next().unwrap();
                let second = iter.next().unwrap();
                let dir = first > second;
                let diff = first.abs_diff(*second);
                let mut last = *second;
                (diff > 0 && diff <= 3)
                    && iter.all(|x| {
                        let diff = x.abs_diff(last);
                        let predicate = (dir == (last > *x)) && (diff <= 3 && diff > 0);
                        last = *x;
                        predicate
                    })
            })
    }
}

pub struct SkipOneIter<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
    skip: usize,
    current_idx: usize,
}

impl<I, T> Iterator for SkipOneIter<I, T>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_idx == self.skip {
            self.iter.next()?;
            self.current_idx += 1; // we've advanced one position
        }

        let item = self.iter.next()?;
        self.current_idx += 1;
        Some(item)
    }
}

impl<I, T> SkipOneIter<I, T>
where
    I: Iterator<Item = T>,
{
    pub fn build(iter: I, skip_idx: usize) -> Self {
        //let iter: std::slice::Iter<'_, T> = v.iter();
        SkipOneIter {
            iter,
            skip: skip_idx,
            current_idx: 0,
        }
    }
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
        assert_eq!(2, count_safe_reports(&reports));
    }

    #[test]
    fn damping() {
        let r1 = Report {
            levels: vec![1, 2, 3, 4, 5],
        };
        assert!(Report::is_safe_dampened(&r1));
        let r2 = Report {
            levels: vec![1, 2, 8, 4, 5],
        };
        assert!(Report::is_safe_dampened(&r2));
        let r3 = Report {
            levels: vec![1, 2, 8, 1, 5],
        };
        assert!(!Report::is_safe_dampened(&r3));
        let r4 = Report {
            levels: vec![8, 2, 3, 4, 5],
        };
        assert!(Report::is_safe_dampened(&r4));
    }

    #[test]
    fn num_safe_reports_dampened() {
        let reports = parse_input(String::from(TEST_INPUT));
        assert_eq!(4, count_safe_reports_with_dampener(&reports));
    }
}
