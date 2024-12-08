fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
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

impl SafetyFeatures for Vec<i32> {
    fn is_sorted(&self) -> bool {
        self.is_sorted_by(|x, y| x.cmp(y).is_gt()) || self.is_sorted_by(|x, y| x.cmp(y).is_lt())
    }

    fn is_safe(&self) -> bool {
        self.windows(2)
            .all(|w| (1..=3).contains(&(w[0] - w[1]).abs()))
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
    parse_input(input)
        .into_iter()
        .map(|line| {
            (0..line.len()).any(|i| {
                let list_with_index_removed: Vec<i32> = line
                    .iter()
                    .enumerate()
                    .filter_map(|(j, k)| if i != j { Some(*k) } else { None })
                    .collect();
                list_with_index_removed.is_sorted() && list_with_index_removed.is_safe()
            })
        })
        .filter(|x| *x)
        .count() as i32
}
