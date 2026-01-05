mod day_1;
use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    let stdout = stdout();
    let distance = day_1::part_1();
    let similarity = day_1::part_2();
    let message = format!(
        "The distance is {}, and the similarity is {}",
        distance, similarity
    );
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, 10, &mut writer).unwrap();
}
