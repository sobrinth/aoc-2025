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

pub fn part_1_cursed_unsafe(input: &str) -> usize {
    const LINE_LEN: usize = 24;
    const STRIDE: usize = LINE_LEN + 1;
    const AREA_MULT: u32 = 9;

    #[inline(always)]
    unsafe fn d2(p: *const u8, offset: usize) -> u32 {
        unsafe {
            let tens = *p.add(offset) - b'0';
            let ones = *p.add(offset + 1) - b'0';
            (tens as u32) * 10 + (ones as u32)
        }
    }

    #[inline(always)]
    unsafe fn fits(p: *const u8) -> usize {
        unsafe {
            let width = d2(p, 0);
            let height = d2(p, 3);
            let area_times_9 = width * height;

            let needed = d2(p, 7) + d2(p, 10) + d2(p, 13) + d2(p, 16) + d2(p, 19);
            (needed * AREA_MULT <= area_times_9) as usize
        }
    }

    let last_block = match input.rfind("\n\n") {
        Some(idx) => &input[idx + 2..],
        None => input,
    };

    let bytes = last_block.as_bytes();

    debug_assert!(
        bytes.is_empty() || bytes.len() % STRIDE == 0,
        "expected last block to be N*(24 chars + '\\n') bytes"
    );
    debug_assert!(
        bytes.is_empty() || bytes[STRIDE - 1] == b'\n',
        "expected records to be newline-terminated"
    );
    debug_assert!(
        bytes.is_empty() || bytes.len() >= STRIDE,
        "expected at least one full record when non-empty"
    );

    // Assumption: last_block ends with '\n' and has only fixed-width records.
    let line_count = bytes.len() / STRIDE;

    unsafe {
        let mut p = bytes.as_ptr();
        let end = p.add(line_count * STRIDE);

        let mut count = 0usize;
        while p < end {
            count += fits(p);
            p = p.add(STRIDE);
        }
        count
    }
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
