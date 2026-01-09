use crate::inputs;
use regex::Regex;

pub fn run() -> String {
    let multiplications = inputs::parse_input_for_day(3, parse_input);
    let sum: u32 = multiplications.iter().map(|(a, b)| a * b).sum();
    format!("The sum is {sum}")
}

fn parse_input(str: String) -> Vec<(u32, u32)> {
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = vec![];

    for (_, [a, b]) in r.captures_iter(&str).map(|c| c.extract()) {
        results.push((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn parse_input_test() {
        let multiplications = parse_input(String::from(TEST_INPUT));
        assert_eq!(vec![(2, 4), (5, 5), (11, 8), (8, 5)], multiplications);
    }
}
