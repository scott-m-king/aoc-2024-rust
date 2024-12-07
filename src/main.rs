mod day1;
mod utils;

fn main() {
    let arg = utils::load_file("data/day1-test.txt");
    println!("Part 1: {}", day1::part1(&arg));
    println!("Part 2: {}", day1::part2(&arg));
}