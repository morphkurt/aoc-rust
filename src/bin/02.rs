advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let items = parse(input);
    let mut sum: i32 = 0;
    for line in items {
        if is_safe(&line) {
            sum += 1;
        }
    }
    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let items = parse(input);
    let mut sum: i32 = 0;
    for line in items {
        if is_safe(&line) {
            sum += 1;
        } else {
            for i in 0..line.len() {
                let mut new_line = line.clone();
                new_line.remove(i);
                if is_safe(&new_line) {
                    sum += 1;
                    break;
                }
            }
        }
    }
    Some(sum.try_into().unwrap())
}


fn is_safe(line: &Vec<i32>) -> bool {
    let  prev_dif = line[1] - line[0];
    let mut found: bool = true;
    for i in 2..line.len() {
        let c = line[i];
        let p = line[i - 1];
        let dif = c - p;
        if  prev_dif * dif < 0 || dif.abs() > 3 || dif == 0 || prev_dif == 0 || prev_dif.abs() > 3{
                found = false;
                break;
        }
        found = found && true;
    }
    return found;
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    // Read the contents of the file
    let mut items: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        // Parse the line
        let v: Vec<i32> = line.split(" ").map(|x: &str| x.parse::<i32>().unwrap()).collect();
        items.push(v);
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
