use regex::Regex;

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")
        .unwrap()
        .find_iter(input)
        .map(|m| m.as_str())
        .map(|line| {
            if let (Some(start), Some(end)) = (line.find("("), line.find(")")) {
                let result: &Vec<&str> = &line[start + 1..end].split(",").collect();
                return (result[0].parse().unwrap(), result[1].parse().unwrap());
            }
            (0, 0)
        })
        .collect::<Vec<(i32, i32)>>()
}

pub fn part1(input: &str) -> i32 {
    parse_input(input)
        .iter()
        .fold(0, |acc, (l, r)| acc + (l * r))
}

pub fn part2(input: &str) -> i32 {
    0
}
