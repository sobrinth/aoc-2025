use std::ops::RangeInclusive;
use itertools::Itertools;

pub fn part_1(input: &(Vec<std::ops::RangeInclusive<u64>>, Vec<u64>)) -> i32 {
    input.1.iter().fold(0, |acc, &i| {
        for r in &input.0 {
            if r.contains(&i) {
                return acc + 1;
            }
        }
        acc
    })
}

pub fn part_2(input: &(Vec<RangeInclusive<u64>>, Vec<u64>)) -> u64 {
    let mut ranges = input.0.clone();
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let res = ranges.iter().fold((0u64, 0), |(mut v, mut cur), r| {
        for ingredient in r.clone() {
            if ingredient > cur {
                v += 1;
                cur = ingredient;
            }
        }
        (v, cur)
    });
    res.0
}

pub fn parse(input: &str) -> (Vec<std::ops::RangeInclusive<u64>>, Vec<u64>) {
    let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();
    (
        fresh_ranges
            .lines()
            .map(|l| {
                let (s, e) = l.split_once('-').unwrap();
                s.parse().unwrap()..=e.parse().unwrap()
            })
            .collect_vec(),
        ingredients
            .lines()
            .map(|l| l.parse().unwrap())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

        assert_eq!(part_1(&parse(s)), 3);
    }
    #[test]
    fn test_part_2() {
        let s = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

        assert_eq!(part_2(&parse(s)), 14);
    }
}
