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

pub fn part_2(input: &str) -> i32 {
    0
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

        assert_eq!(part_2(s), 0);
    }
}
