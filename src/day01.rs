use std::io::BufRead;

pub fn part1<R: BufRead>(mut input: R) -> String {
    let mut buf = String::new();
    let mut current_lock_position = 50;
    let mut result = 0;

    while let Ok(num_bytes_read) = input.read_line(&mut buf) {
        if num_bytes_read == 0 {
            break;
        }

        {
            let (letter, number_of_turns) = buf.split_at(1);
            let letter = letter.chars().next().unwrap();
            let number_of_turns: i32 = number_of_turns.trim().parse().unwrap();

            // Actual solution logic is here:
            let meaningfull_number_of_turns = number_of_turns % 100;
            if letter == 'L' {
                current_lock_position -= meaningfull_number_of_turns;
                current_lock_position += 100;
                current_lock_position %= 100;
            } else if letter == 'R' {
                current_lock_position += meaningfull_number_of_turns;
                current_lock_position %= 100;
            } else {
                panic!(
                    "Error: first symbol in each of the lines should be either L for the left turn or R for the right turn"
                );
            }

            if current_lock_position == 0 {
                result += 1;
            }
        }

        buf.clear();
    }

    result.to_string()
}

pub fn part2(input: &str) -> String {
    "lol".to_string()
}
