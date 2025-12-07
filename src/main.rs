#![allow(dead_code)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod shared;

aoc_main::main! {
    year 2025;
    day01 => part_1, part_2;
    day02: generate => part_1, part_2;
    day03 => part_1, part_2;
    day04:  parse => part_1, part_2;
    day05 => part_1, part_2_optimized;
    day06 => part_1, part_2;
    day07 => part_1, part_2;
}
