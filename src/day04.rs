use crate::day04::Warehouse::{Empty, PaperRoll};
use crate::shared::Grid;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_1(input: &Grid<Warehouse>) -> i32 {
    let mut count = 0;
    for x in 0..input.width {
        for y in 0..input.height {
            if input[(x, y)] == Empty {
                continue;
            }
            if input.count_all_neighbours(x, y, PaperRoll) < 4 {
                count += 1;
            }
        }
    }
    count
}

pub fn part_2(input: &Grid<Warehouse>) -> i32 {
    let mut count = 0;
    let mut state: HashMap<(isize, isize), usize> = HashMap::new();

    for x in 0..input.width {
        for y in 0..input.height {
            if input[(x, y)] == Empty {
                continue;
            }
            state.insert((x, y), input.count_all_neighbours(x, y, PaperRoll));
        }
    }

    let mut yeet = vec![];
    let mut done = false;
    while !done {
        state.retain(|i, n_count| {
            if *n_count < 4 {
                yeet.push(*i);
                count += 1;
                false
            } else {
                true
            }
        });
        done = yeet.is_empty();

        for i in yeet.drain(..) {
            decrement(&mut state, i);
        }
    }

    count
}

fn decrement(state: &mut HashMap<(isize, isize), usize>, pos: (isize, isize)) {
    for dx in [-1, 0, 1].iter() {
        for dy in [-1, 0, 1].iter() {
            if *dx == 0 && *dy == 0 {
                continue;
            }
            let possible_neighbour = (pos.0 + dx, pos.1 + dy);
            if let Some(n) = state.get_mut(&possible_neighbour) {
                *n -= 1;
            }
        }
    }
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

        assert_eq!(part_2(&parse(s)), 43);
    }
}
