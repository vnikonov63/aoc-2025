use std::{env, fs::File, io::BufReader};

mod day01;

fn main() {
    let mut args = env::args().skip(1);

    let day: u8 = args
        .next()
        .expect("Usage: aot-2025 <day> <part>")
        .parse()
        .expect("Usage: day must be a number");
    let part: u8 = args
        .next()
        .expect("Usage:: aot-2025 <day> <part>")
        .parse()
        .expect("Usage: part must be a number");
    if part != 1 && part != 2 {
        panic!("Usage: part must be wither 1 or 2");
    }

    let input_file_path = format!("input/day{:02}.txt", day);
    let input_file = File::open(&input_file_path)
        .unwrap_or_else(|_| panic!("Error: could not open {}", input_file_path));

    let reader = BufReader::new(input_file);

    match (day, part) {
        (1, 1) => println!("{}", day01::part1(reader)),
        (1, 2) => println!("{}", day01::part1(reader)),
        _ => panic!("not implemented yet"),
    }
}
