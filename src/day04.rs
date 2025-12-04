use crate::day04::Warehouse::{Empty, PaperRoll};
use crate::shared::Grid;
use itertools::Itertools;

pub fn part_1(input: &Grid<Warehouse>) -> i32 {
    let mut count = 0;
    for x in 0..input.width {
        for y in 0..input.height {
            if input[(x, y)] == Empty {
                continue;
            }
            if input.count_neighbours_eight(x, y, PaperRoll) < 4 {
                count += 1;
            }
        }
    }
    count
}

pub fn part_2(input: &str) -> i32 {
    todo!()
}

pub fn parse(input: &str) -> Grid<Warehouse> {
    let height = input.lines().count();
    let content = input.lines().fold(vec![], |mut acc, l| {
        let mut mapped_line = l
            .chars()
            .map(|c| match c {
                '@' => PaperRoll,
                '.' => Empty,
                _ => panic!("Invalid warehouse character: {}", c),
            })
            .collect_vec();
        acc.append(&mut mapped_line);
        acc
    });
    Grid {
        height: height as isize,
        width: content.len() as isize / height as isize,
        content,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Warehouse {
    PaperRoll,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(part_1(&parse(s)), 13);
    }
    #[test]
    fn test_part_2() {
        let s = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(part_2(s), 1111111);
    }
}
