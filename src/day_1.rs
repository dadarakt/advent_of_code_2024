use crate::inputs;

pub fn part_1() -> u32 {
    let (l, r) = inputs::parse_input_for_day(1, parse_input);
    compute_distance(l, r)
}

pub fn part_2() -> u32 {
    let (l, r) = inputs::parse_input_for_day(1, parse_input);
    compute_similarity(l, r)
}

fn parse_input(input: String) -> (Vec<u32>, Vec<u32>) {
    let mut l: Vec<u32> = Vec::new();
    let mut r: Vec<u32> = Vec::new();

    for line in input.lines() {
        let (l_str, r_str) = line.trim().split_once(' ').expect("Invalid input");
        l.push(l_str.trim().parse().expect("Invalid number"));
        r.push(r_str.trim().parse().expect("Invalid number"));
    }

    l.sort();
    r.sort();

    (l, r)
}

fn compute_distance(l: Vec<u32>, r: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..(l.len()) {
        sum += l[i].abs_diff(r[i]);
    }
    sum
}

fn compute_similarity(l: Vec<u32>, r: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    let mut right_sum = 0;
    l.iter().for_each(|v| {
        right_sum = 0;
        r.iter().for_each(|r_val| {
            if r_val == v {
                right_sum += 1;
            }
        });
        sum += v * right_sum;
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn parse_input_test() {
        let (l, r) = parse_input(String::from(TEST_INPUT));
        assert_eq!(l.len(), r.len());
    }

    #[test]
    fn distance_test() {
        let (l, r) = parse_input(String::from(TEST_INPUT));
        let sum = compute_distance(l, r);
        assert_eq!(11, sum);
    }

    #[test]
    fn similarity_test() {
        let (l, r) = parse_input(String::from(TEST_INPUT));
        let sum = compute_similarity(l, r);
        assert_eq!(31, sum);
    }
}
