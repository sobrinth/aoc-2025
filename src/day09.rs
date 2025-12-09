use itertools::Itertools;

pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|l| {
            l.split_once(",").map(|(x, y)| {
                let x = x.parse::<u64>().unwrap();
                let y = y.parse::<u64>().unwrap();
                (x, y)
            })
        })
        .tuple_combinations()
        .map(|(a, b)| {
            (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
        })
        .max()
        .unwrap()
}

pub fn part_2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

        assert_eq!(part_1(s), 50);
    }
    #[test]
    fn test_part_2() {
        let s = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

        assert_eq!(part_2(s), 0);
    }
}
