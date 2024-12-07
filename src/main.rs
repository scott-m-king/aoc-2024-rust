#![allow(unused_variables)]
#![allow(dead_code)]
mod utils;
mod day2;

fn main() {
    let arg = utils::load_file("data/day-2.txt");
    println!("Part 1: {}", day2::part1(&arg));
    println!("Part 2: {}", day2::part2(&arg));
}