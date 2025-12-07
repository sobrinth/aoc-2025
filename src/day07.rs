use std::collections::HashSet;

pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .fold((0, HashSet::new()), |(mut splits, mut acc), l| {
            l.chars().enumerate().for_each(|(i, c)| {
                match c {
                    'S' => {
                        acc.insert(i);
                    }
                    '.' => {}
                    '^' => {
                        if acc.contains(&i) {
                            splits += 1;
                            acc.remove(&i);
                            acc.insert(i + 1);
                            acc.insert(i - 1);
                        }
                    }
                    _ => {}
                };
            });

            (splits, acc)
        })
        .0
}

pub fn part_2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

        assert_eq!(part_1(s), 21);
    }
    #[test]
    fn test_part_2() {
        let s = "input";

        assert_eq!(part_2(s), 0);
    }
}
