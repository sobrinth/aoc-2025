use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let sections = input.split("\n\n").collect_vec();

    let gift_sizes = sections[..sections.len() - 1]
        .iter()
        .map(|g| g.bytes().filter(|&b| b == b'#').count())
        .collect_vec();

    sections[sections.len() - 1]
        .lines()
        .filter(|l| {
            let (dims, todo) = l.split_once(':').unwrap();
            let (x, y) = dims.split_once('x').unwrap();
            let area = x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap();
            let needed: usize = todo
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .enumerate()
                .map(|(i, x)| gift_sizes[i] * x)
                .sum();
            // This seems to work on the actual input??? The gifts either fit with big margin or not at all
            needed <= area
        })
        .count()
}

pub fn part_2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

        assert_eq!(part_1(s), 2);
    }
    #[test]
    fn test_part_2() {
        let s = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

        assert_eq!(part_2(s), 0);
    }
}
