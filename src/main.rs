mod day01;
mod day02;
mod day03;
mod day04;
mod shared;

aoc_main::main! {
    year 2025;
    day01 => part_1, part_2;
    day02: generate => part_1, part_2;
    day03 => part_1, part_2;
}
