use std::io::BufRead;

fn is_reachable(matrix: &[Vec<bool>], row: usize, col: usize) -> bool {
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut counter = 0u8;

    for (dr, dc) in offsets {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;

        if nr < 0 || nc < 0 {
            continue;
        }

        let nr = nr as usize;
        let nc = nc as usize;

        if nr >= matrix.len() || nc >= matrix[0].len() {
            continue;
        }

        if matrix[nr][nc] {
            counter += 1;
        }
    }

    counter < 4
}

pub fn part1<R: BufRead>(mut input: R) -> String {
    let mut buf = String::new();
    let mut matrix: Vec<Vec<bool>> = Vec::new();

    while let Ok(n) = input.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        matrix.push(
            buf.trim()
                .chars()
                .map(|c| match c {
                    '@' => true,
                    '.' => false,
                    _ => panic!("invalid char"),
                })
                .collect(),
        );
        buf.clear();
    }

    let mut result = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] && is_reachable(&matrix, row, col) {
                result += 1;
            }
        }
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
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let cursor = Cursor::new(input.as_bytes());
        let result = part1(cursor);
        assert_eq!(result, "13");
    }
}
