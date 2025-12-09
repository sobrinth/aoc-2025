use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Tile {
    x: u64,
    y: u64,
}

pub fn part_1(input: &str) -> u64 {
    parse_tiles(input)
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> u64 {
    let mut tiles = parse_tiles(input);

    let mut possible_rectangles = Vec::with_capacity(tiles.len() * tiles.len());

    tiles.iter().tuple_combinations().for_each(|(&a, &b)| {
        possible_rectangles.push((a, b, (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)))
    });

    // add the first tile to the end to make it easier to "wrap around"
    tiles.push(tiles[0]);

    possible_rectangles
        .iter()
        .map(|(p1, p2, area)| {
            /*
               Check if the rectangle is fully inside the bounding box of the "floor plan"
               So: the rectangle is not allowed to intersect with any of the lines of the "floor plan"
               and needs to be fully contained within the bounding box
            */
            let [bx_min, bx_max, by_min, by_max] = bounding_box(p1, p2);
            for w in tiles.windows(2) {
                let [linex_min, linex_max, liney_min, liney_max] = bounding_box(&w[0], &w[1]);
                // Handle vertical line intersection
                if linex_min == linex_max {
                    if linex_min > bx_min && linex_max < bx_max {
                        if !(liney_max <= by_min || liney_min >= by_max) {
                            return None;
                        }
                    }
                }

                // Handle horizontal line intersection
                if liney_min == liney_max {
                    if liney_min > by_min && liney_max < by_max {
                        if !(linex_max <= bx_min || linex_min >= bx_max) {
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

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .lines()
        .filter_map(|l| {
            l.split_once(",").map(|(x, y)| {
                let x = x.parse::<u64>().unwrap();
                let y = y.parse::<u64>().unwrap();
                Tile { x, y }
            })
        })
        .collect()
}

fn bounding_box(p1: &Tile, p2: &Tile) -> [u64; 4] {
    [
        p1.x.min(p2.x),
        p1.x.max(p2.x),
        p1.y.min(p2.y),
        p1.y.max(p2.y),
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
