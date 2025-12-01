use std::{env, fs};

mod day01;

fn main() {
    let mut args = env::args();

    let day: u8 = args
        .nth(1)
        .expect("Usage: aot-2025 <day> <part>")
        .parse()
        .expect("Usage: day must be a number");
    let part: u8 = args
        .nth(2)
        .expect("Usage:: aot-2025 <day> <part>")
        .parse()
        .expect("Usage: part must be a number");
    if part != 1 && part != 2 {
        panic!("Usage: part must be wither 1 or 2");
    }

    let input_file_path = format!("input/day{:02}.txt", day);
    let input_content = fs::read_to_string(&input_file_path)
        .unwrap_or_else(|_| panic!("Error: could not read {}", input_file_path));

    match (day, part) {
        (1, 1) => println!("{}", day01::part1(&input_content)),
        (1, 2) => println!("{}", day01::part1(&input_content)),
        _ => panic!("not implemented yet"),
    }
}
