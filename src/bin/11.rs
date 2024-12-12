use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones: Vec<u64> = parse(input);
    let mut cache: HashMap<u64, Vec<u64>> = HashMap::new();
    for _ in 0..25 {
        let mut interim_stones: Vec<u64> = Vec::new();
        for stone in stones.iter() {
            if cache.contains_key(stone) {
                interim_stones.extend(cache.get(stone).unwrap().iter());
                continue;
            } else {
                let s = format!("{}", stone);
                if *stone == 0 {
                    interim_stones.push(1);
                    cache.insert(0, vec![1]);
                } else if s.len() % 2 == 0 {
                    let left = s.chars().take(s.len() / 2).collect::<String>();
                    let right = s.chars().skip(s.len() / 2).collect::<String>();
                    interim_stones.push(left.parse::<u64>().unwrap());
                    interim_stones.push(right.parse::<u64>().unwrap());
                    cache.insert(
                        *stone,
                        vec![left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()],
                    );
                } else {
                    interim_stones.push(stone * 2024);
                    cache.insert(*stone, vec![stone * 2024]);
                }
            }
        }
        stones = interim_stones;
    }

    Some(stones.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let digits: Vec<u64> = parse(input);

    //let mut stones = parse(input);
    let mut stones: HashMap<u64, u64> = HashMap::new();

    for stone in digits.iter() {
        stones.insert(*stone, 1);
    }

    for _ in 0..75 {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for (stone_label, stone_count) in stones.iter_mut() {
            let s = format!("{}", stone_label);
            if *stone_label == 0 {
                *new_stones.entry(1).or_insert(0) += *stone_count;
            } else if s.len() % 2 == 0 {
                let left = s.chars().take(s.len() / 2).collect::<String>();
                let right = s.chars().skip(s.len() / 2).collect::<String>();
                *new_stones.entry(left.parse::<u64>().unwrap()).or_insert(0) += *stone_count;
                *new_stones.entry(right.parse::<u64>().unwrap()).or_insert(0) += *stone_count;
            } else {
                *new_stones.entry(stone_label * 2024).or_insert(0) += *stone_count;
            }
        }
        stones = new_stones;
    }

    Some(stones.values().sum())
}

fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
