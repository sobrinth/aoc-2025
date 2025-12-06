use itertools::Itertools;

pub fn part_1(input: &str) -> u64 {
    let rows = input
        .lines()
        .map(|r| r.split_ascii_whitespace().collect_vec())
        .collect_vec();
    let row_len = rows[0].len();

    let mut count = 0;
    for i in 0..row_len {
        match rows[4][i] {
            "+" => {
                count += rows[0][i].parse::<u64>().unwrap()
                    + rows[1][i].parse::<u64>().unwrap()
                    + rows[2][i].parse::<u64>().unwrap()
                    + rows[3][i].parse::<u64>().unwrap();
            }
            "*" => {
                count += rows[0][i].parse::<u64>().unwrap()
                    * rows[1][i].parse::<u64>().unwrap()
                    * rows[2][i].parse::<u64>().unwrap()
                    * rows[3][i].parse::<u64>().unwrap();
            }
            _ => panic!("Invalid row op"),
        }
    }

    count
}

pub fn part_2(input: &str) -> u64 {
    let mut cols = vec![];
    let mut count = 0;

    /*
    Note for me: because I'll forget what I did here in approx 10 mins.
    Treat the input as a grid of numbers.
    Iterate over the input line by line and store the "number" in the column in the vector at idx = col-nr
    The operator is always in the 'first' column of a new calculation -> take until the first column that is 0 again
     */
    for l in input.lines() {
        for (idx, c) in l.chars().enumerate() {
            if cols.len() == idx {
                cols.push(0);
            }
            if let Some(n) = c.to_digit(10) {
                cols[idx] *= 10; // the new digit will be added as the least-significant, so move everything up one
                cols[idx] += n as u64;
            } else if c == '+' {
                count += cols[idx..].iter().take_while(|&&x| x != 0).sum::<u64>();
            } else if c == '*' {
                count += cols[idx..].iter().take_while(|&&x| x != 0).product::<u64>();
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

        assert_eq!(part_1(s), 4277556);
    }
    #[test]
    fn test_part_2() {
        let s = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

        assert_eq!(part_2(s), 3263827);
    }
}
