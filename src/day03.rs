use std::io::BufRead;

pub fn part1<R: BufRead>(mut input: R) -> String {
    let mut buf = String::new();
    let mut result = 0;

    while let Ok(num_bytes_read) = input.read_line(&mut buf) {
        if num_bytes_read == 0 {
            break;
        }

        let mut best_so_far: u32 = buf.chars().next().unwrap().to_digit(10).unwrap();
        let mut line_result: u32 = 0;

        for ch in buf.trim().chars().skip(1).map(|x| x.to_digit(10).unwrap()) {
            line_result = line_result.max(best_so_far * 10 + ch);
            best_so_far = best_so_far.max(ch)
        }

        result += line_result;
        buf.clear();
    }

    result.to_string()
}

pub fn part2<R: BufRead>(mut input: R) -> String {
    let mut buf = String::new();
    let mut result = 0;

    while let Ok(num_bytes_read) = input.read_line(&mut buf) {
        if num_bytes_read == 0 {
            break;
        }
        buf.clear();
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_part1() {
        let input = "987654321111111\n811111111111119\n34234234234278\n818181911112111";
        let cursor = Cursor::new(input.as_bytes());
        let result = part1(cursor);
        assert_eq!(result, "357");
    }
}
