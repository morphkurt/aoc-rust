use std::collections::HashMap;


advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u32> {
    let (mut left, mut right)= parse(_input);

    left.sort();
    right.sort();
    let mut sum: i32 = 0;
    for i in 0..left.len() {
          sum+=(left[i]-right[i]).abs()
    }
    Some(sum.try_into().unwrap())
}

pub fn part_two(_input: &str) -> Option<u32> {
    let (left, right) = parse(_input);
    let mut map = HashMap::new();
    for ele in right {
        *map.entry(ele).or_insert(0) += 1;
    }
    let sum: i32 = left.iter().map(|ele| ele * map.get(ele).unwrap_or(&0)).sum();
    Some(sum.try_into().unwrap())
}

fn parse(input: &str) -> (Vec<i32> , Vec<i32>) {
    // Read the contents of the file
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        // Parse the line
        let items: Vec<i32> = line.split("   ").map(|x: &str| x.parse::<i32>().unwrap()).collect();
        left.push(items[0]);
        right.push(items[1]);

    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

}
