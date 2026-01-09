use crate::inputs;
use regex::Regex;

pub fn run() -> String {
    let sum_part_1 = part_1();
    let sum_part_2 = part_2();
    format!("The sum of part 1 is {sum_part_1} and the sum of part 2 is {sum_part_2}")
}

fn part_1() -> u32 {
    let multiplications = inputs::parse_input_for_day(3, parse_simple_multiplications);
    multiplications.iter().map(|(a, b)| a * b).sum()
}

fn part_2() -> u32 {
    let multiplications = inputs::parse_input_for_day(3, parse_multiplications_with_condition);
    multiplications.iter().map(|(a, b)| a * b).sum()
}

/**
* Parses all multiplications from the input
*/
fn parse_simple_multiplications(str: String) -> Vec<(u32, u32)> {
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = vec![];

    for (_, [a, b]) in r.captures_iter(&str).map(|c| c.extract()) {
        results.push((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));
    }

    results
}

fn parse_multiplications_with_condition(str: String) -> Vec<(u32, u32)> {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"(do\(\))").unwrap();
    let dont_regex = Regex::new(r"(don't\(\))").unwrap();
    let mut results = vec![];

    let mut dos = do_regex
        .captures_iter(&str)
        .map(|cap| cap.get(0).unwrap().start());

    let mut donts = dont_regex
        .captures_iter(&str)
        .map(|cap| cap.get(0).unwrap().start());

    let mut next_do = dos.next();
    let mut last_do_idx = 0;
    let mut next_dont = donts.next();
    let mut last_dont_idx = 0;
    let caps = mul_regex.captures_iter(&str).filter_map(|c| {
        let start = c.get(0).unwrap().start();

        while next_do.is_some_and(|idx| idx < start) {
            last_do_idx = next_do.unwrap();
            next_do = dos.next();
        }

        while next_dont.is_some_and(|idx| idx < start) {
            last_dont_idx = next_dont.unwrap();
            next_dont = donts.next();
        }

        match last_do_idx >= last_dont_idx {
            true => Some(c.extract()),
            false => None,
        }
    });

    for (_, [a, b]) in caps {
        results.push((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_PART_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn day_1_test() {
        let multiplications = parse_simple_multiplications(String::from(TEST_INPUT));
        assert_eq!(vec![(2, 4), (5, 5), (11, 8), (8, 5)], multiplications);
    }

    #[test]
    fn day_2_test() {
        let multiplications = parse_multiplications_with_condition(String::from(TEST_INPUT_PART_2));
        assert_eq!(vec![(2, 4), (8, 5)], multiplications);
    }
}
