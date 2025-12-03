use std::io::BufRead;

pub fn part1<R: BufRead>(mut input: R) -> String {
    let mut buf = String::new();
    let mut result: i128 = 0;

    while let Ok(num_bytes_read) = input.read_line(&mut buf) {
        if num_bytes_read == 0 {
            break;
        }

        // solution logic is here
        {
            let ranges: Vec<String> = buf.split(',').map(|x| x.to_string()).collect();
            for range in ranges {
                let [mut left, mut right]: [String; 2] = range
                    .split('-')
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Error: a range must have a left an a right bound");

                if left.len() % 2 == 1 && right.len() % 2 == 0 && left.len() == right.len() {
                    continue;
                }

                if left.len() % 2 == 1 {
                    left = format!("1{}", "0".repeat(left.len()));
                }

                if right.len() % 2 == 1 {
                    right = "9".repeat(right.len() - 1);
                }

                let leftmid = left.len() / 2;
                let rightmid = right.len() / 2;

                let (leftleft, rightleft) = left.split_at(leftmid);
                let (leftleft, rightleft) = (leftleft.to_string(), rightleft.to_string());

                let (leftright, rightright) = right.split_at(rightmid);
                let (leftright, rightright) = (leftright.to_string(), rightright.to_string());

                if left == right && leftleft == rightright {
                    result += left
                        .parse::<i128>()
                        .expect("Error: range endpoints should be a number");
                    continue;
                }

                if leftleft == leftright {
                    result += left.parse::<i128>().unwrap();
                }

                if rightleft == rightright {
                    result += right.parse::<i128>().unwrap();
                }

                let distace =
                    rightleft.parse::<i128>().unwrap() - leftleft.parse::<i128>().unwrap() - 1;
                for i in 1..distace {
                    result += (rightleft.parse::<i128>().unwrap() + i)
                        * 10i128.pow(rightleft.len() as u32)
                        + (rightleft.parse::<i128>().unwrap() + i)
                }
            }
        }

        buf.clear();
    }

    result.to_string()
}

pub fn part2<R: BufRead>(mut input: R) -> String {
    let mut buf = String::new();
    let mut current_lock_position = 50;
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
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let cursor = Cursor::new(input.as_bytes());
        let result = part1(cursor);
        assert_eq!(result, "1227775554");
    }
}
