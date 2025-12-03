use itertools::Itertools;

pub(crate) fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .map(|l| {
            l.windows(2)
                .fold(vec![0, 0], |mut v, n| {
                    for i in 0..2 {
                        if n[i] > v[i] {
                            v.truncate(i);
                            v.append(&mut n[i..].to_vec());
                            break;
                        }
                    }
                    v
                })
                .iter()
                .fold(0u32, |acc, n| acc * 10 + *n)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(part_1(s), 357);
    }
}
