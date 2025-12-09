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
        .map(|(a, b)| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> u64 {
    let mut tiles = input
        .lines()
        .filter_map(|l| {
            l.split_once(",").map(|(x, y)| {
                let x = x.parse::<u64>().unwrap();
                let y = y.parse::<u64>().unwrap();
                (x, y)
            })
        })
        .collect_vec();

    let mut possible_rectangles = Vec::with_capacity(tiles.len() * tiles.len());

    tiles.iter().tuple_combinations().for_each(|(&a, &b)| {
        possible_rectangles.push((a, b, (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)))
    });

    // add the first tile to the end to make it easier to "wrap around"
    tiles.push(tiles[0]);

    possible_rectangles
        .iter()
        .map(|(p1, p2, area)| {
            let [bbx_min, bbx_max, bby_min, bby_max] = bounding_box(p1, p2);
            for w in tiles.windows(2) {
                let [linex_min, linex_max, liney_min, liney_max] = bounding_box(&w[0], &w[1]);
                // Handle vertical line intersection
                if linex_min == linex_max {
                    if linex_min > bbx_min && linex_max < bbx_max {
                        if !(liney_max <= bby_min || liney_min >= bby_max) {
                            return None;
                        }
                    }
                }

                // Handle horizontal line intersection
                if liney_min == liney_max {
                    if liney_min > bby_min && liney_max < bby_max {
                        if !(linex_max <= bbx_min || linex_min >= bbx_max) {
                            return None;
                        }
                    }
                }
            }
            Some(*area)
        })
        .filter_map(|area| area)
        .max()
        .unwrap()
}

fn bounding_box(p1: &(u64, u64), p2: &(u64, u64)) -> [u64; 4] {
    [
        p1.0.min(p2.0),
        p1.0.max(p2.0),
        p1.1.min(p2.1),
        p1.1.max(p2.1),
    ]
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

        assert_eq!(part_2(s), 24);
    }
}
