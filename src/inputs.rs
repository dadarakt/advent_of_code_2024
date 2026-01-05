use std::fs;
pub fn parse_input_for_day<T>(day: u32, parsing_fun: fn(String) -> T) -> T {
    let path = format!("inputs/day_{}.txt", day);
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    parsing_fun(contents)
}
