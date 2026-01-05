use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello wurstikus!");

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, 10, &mut writer).unwrap();
}
