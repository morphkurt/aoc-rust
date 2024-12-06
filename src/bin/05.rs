use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (map, pages) = parse(input);
    let mut correct: Vec<&Vec<i32>> = Vec::new();
    for item in pages.iter() {
        if is_ordered(&map, item) {
            correct.push(item);
        }
    }
    let mut sum = 0;
    for item in correct.iter() {
        let middle = item.len() / 2;
        sum += item[middle];
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, pages) = parse(input);
    let mut sum = 0;
    for item in pages.iter() {
        if !is_ordered(&map, item) {
            let mut list = item.clone();
            list.sort_by(|a, b| {
                if map.contains_key(b) {
                    let b_dep = map.get(b).unwrap();
                    if b_dep.contains(a) {
                        return std::cmp::Ordering::Less;
                    } else {
                        return std::cmp::Ordering::Equal;
                    }
                }
                std::cmp::Ordering::Equal
            });
            let middle = item.len() / 2;
            sum += list[middle];
        }
    }
    Some(sum as u32)
}

fn is_ordered(map: &HashMap<i32, Vec<i32>>, item: &Vec<i32>) -> bool {
    for (i, _) in item.iter().enumerate() {
        if map.contains_key(&item[i]) {
            let dep = map.get(&item[i]).unwrap();
            if dep.len() > 0 {
                for j in i + 1..item.len() {
                    if dep.contains(&item[j]) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn parse(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    // Read the contents of the file
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut result: Vec<Vec<i32>> = Vec::new();

    let splitted: Vec<&str> = input.split("\n\n").collect();

    for line in splitted[0].lines() {
        let mut parts = line.split("|");
        let value = parts.next().unwrap().parse::<i32>().unwrap();
        let key = parts.next().unwrap().parse::<i32>().unwrap();
        map.entry(key).or_insert_with(Vec::new).push(value);
    }

    for line in splitted[1].lines() {
        let mut parts = line.split(",");
        let mut vec: Vec<i32> = Vec::new();
        while let Some(part) = parts.next() {
            vec.push(part.parse::<i32>().unwrap());
        }
        result.push(vec);
    }
    (map, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
