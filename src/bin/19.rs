use std::collections::{HashMap, HashSet};


advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let ( towels, designs) = parse(input);
    let mut memo = HashMap::new();

    Some(designs
        .iter()
        .filter(|&designs| count_possible(&designs, &towels, &mut memo) > 0)
        .collect::<Vec<_>>()
        .len() as u32)
}

fn count_possible<'a>(pattern: &'a str, towels: &HashSet<&str>, memo: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(&c) = memo.get(pattern) { return c; }

    if pattern.trim().is_empty() { return 1; }

    let mut count = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            count += count_possible(&pattern[towel.len()..], &towels, memo);
        }
    }

    *memo.entry(pattern).or_insert(0) += count;
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let ( towels, designs) = parse(input);
    let mut memo = HashMap::new();

    Some(designs
        .iter()
        .map(|&designs| count_possible(&designs, &towels, &mut memo))
        .sum::<u64>())
}

fn parse(input: &str) -> (HashSet<&str>, Vec<&str>){
    let spllited = input.split("\n\n").collect::<Vec<&str>>();
    (spllited[0].split(", ").collect::<HashSet<&str>>(), spllited[1].split("\n").collect::<Vec<&str>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
