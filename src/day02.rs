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
            let ranges: Vec<String> = buf.trim().split(',').map(|x| x.to_string()).collect();
            for range in ranges {
                let [mut left, mut right]: [String; 2] = range
                    .split('-')
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Error: a range must have a left an a right bound");

                if left.len() % 2 == 1 && right.len() % 2 == 1 && left.len() == right.len() {
                    continue;
                }

                if left.len() % 2 == 1 {
                    left = format!("1{}", "0".repeat(left.len()));
                }

                if right.len() % 2 == 1 {
                    right = "9".repeat(right.len() - 1);
                }

                if left.len() > right.len() {
                    continue;
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

                let l = left
                    .parse::<i128>()
                    .expect("Error: range endpoints should be a number");
                let r = right
                    .parse::<i128>()
                    .expect("Error: range engpoints should be a number");

                let mut ll = leftleft.parse::<i128>().unwrap();
                let mut rl = rightleft.parse::<i128>().unwrap();
                let mut lr = leftright.parse::<i128>().unwrap();
                let mut rr = rightright.parse::<i128>().unwrap();

                let mut start = 0;
                let mut end = lr - ll + 1;
                if rl > ll {
                    start += 1;
                }
                if lr > rr {
                    end -= 1;
                }

                for i in start..end {
                    result += (ll + i) * 10i128.pow(rightleft.len() as u32) + (ll + i)
                }
            }
        }

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
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let cursor = Cursor::new(input.as_bytes());
        let result = part1(cursor);
        assert_eq!(result, "1227775554");
    }
    #[test]
    fn test_part2() {
        let input = "9191896883-9191940271";
        let cursor = Cursor::new(input.as_bytes());
        let result = part1(cursor);
        assert_eq!(result, "0");
    }
    #[test]
    fn test_part3() {
        let input = "0-100";
        let cursor = Cursor::new(input.as_bytes());
        let result = part1(cursor);
        assert_eq!(
            result,
            (11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99).to_string()
        );
    }
    //#[test]
    //fn test_part4() {
    //    let input = "90-1213";
    //    let cursor = Cursor::new(input.as_bytes());
    //    let result = part1(cursor);
    //    assert_eq!(result, (99 + 1010 + 1111 + 1212).to_string());
    //}
}

// 9191896883-9191940271
