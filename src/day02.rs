use itertools::Itertools;

pub fn generate(input: &str) -> Vec<(i64, i64)> {
    input
        .split(",")
        .map(|r| r.split_once("-").unwrap())
        .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
        .collect_vec()
}
pub fn part_1(input: &Vec<(i64, i64)>) -> i64 {
    input
        .iter()
        .fold(0, |mut acc, (start, end)| {
            for i in *start..=*end {
                if i.to_string().len() % 2 == 0 {
                    let i_str = i.to_string();
                    let (l, r) = i_str.split_at(i_str.len() / 2);
                    if l == r {
                        acc += i
                    }
                }
            }
            acc
        })
}

pub fn part_2(input: &Vec<(i64, i64)>) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(part_1(&generate(s)), 1227775554);
    }

    #[test]
    fn test_part_2() {
        let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(part_2(&generate(s)), 4174379265);
    }

}
