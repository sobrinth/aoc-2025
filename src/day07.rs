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

pub fn part_1_alt(input: &str) -> i32 {
    let len = input.lines().next().unwrap().len();
    input
        .lines()
        .fold((0, vec![false; len]), |(mut splits, mut acc), l| {
            for col in 0..len {
                match l.as_bytes()[col] {
                    b'S' => acc[col] = true,
                    b'^' => {
                        if acc[col] {
                            acc[col] = false;
                            acc[col - 1] = true;
                            acc[col + 1 ] = true;
                            splits += 1;
                        }
                    }
                    _ => {}
                }
            }

            (splits, acc)
        })
        .0
}

pub fn part_2(input: &str) -> u64 {
    let len = input.lines().next().unwrap().len();
    input
        .lines()
        .fold(vec![0; len], |mut acc, l| {
            for col in 0..len {
                match l.as_bytes()[col] {
                    b'S' => acc[col] = 1,
                    b'^' => {
                        // no out-of-bounds checking as we only ever get to the edges at the end
                        acc[col - 1] += acc[col];
                        acc[col + 1] += acc[col];
                        acc[col] = 0;
                    }
                    _ => {}
                }
            }
            acc
        })
        .iter()
        .sum()
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

        assert_eq!(part_2(s), 40);
    }
}
