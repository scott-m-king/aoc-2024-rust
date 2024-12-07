fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .map(|x| {
            x.split_whitespace()
                .filter_map(|y| y.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

trait SafetyFeatures {
    fn is_sorted(&self) -> bool;
    fn is_safe(&self) -> bool;
}

impl SafetyFeatures for &Vec<i32> {
    fn is_sorted(&self) -> bool {
        self.is_sorted_by(|x, y| x.cmp(y).is_gt()) || self.is_sorted_by(|x, y| x.cmp(y).is_lt())
    }

    fn is_safe(&self) -> bool {
        self.windows(2)
            .all(|window| (1..=3).contains(&(window[0] - window[1]).abs()))
    }
}

pub fn part1(input: &str) -> i32 {
    parse_input(input)
        .iter()
        .map(|line| line.is_sorted() && line.is_safe())
        .filter(|x| *x)
        .count() as i32
}

pub fn part2(input: &str) -> i32 {
    0
}
