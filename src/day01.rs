pub fn part_1(input: &str) -> i32 {
    let mut result_count = 0;
    input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(dir, num)| {
            let num = num.parse::<i32>().unwrap();
            (dir, num)
        })
        .fold(50, |mut acc, (dir, num)| {
            if dir == "L" {
                acc = (acc - num) % 100;
            } else {
                acc = (acc + num) % 100;
            }
            if acc == 0 {
                result_count += 1;
            };
            acc
        });
    result_count
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(dir, num)| {
            if dir == "L" {
                num.parse::<i32>().unwrap() * -1
            } else {
                num.parse::<i32>().unwrap()
            }
        })
        .fold((50i32, 0), |(dial, sum), num| {
            let new = dial.wrapping_add(num);

            (
                new.rem_euclid(100),
                sum + (new <= 0 && dial != 0) as i32 + (new.signum() * new / 100),
            ) // Add 1 if dial goes below 0, Add how many times we overshot
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part_1(s), 3);
    }
    #[test]
    fn test_part_2() {
        let s = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part_2(s), 6);
    }
}
