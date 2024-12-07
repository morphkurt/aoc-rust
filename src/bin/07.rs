advent_of_code::solution!(7);

fn add_fn(a: u64, b: u64) -> u64 {
    a + b
}

fn mul_fn(a: u64, b: u64) -> u64 {
    a * b
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

const OPCODES: [fn(u64, u64) -> u64; 2] = [add_fn, mul_fn];
const OPCODES_PART2: [fn(u64, u64) -> u64; 3] = [concat, add_fn, mul_fn];

pub fn part_one(input: &str) -> Option<u64> {
    let (totals, numbers) = parse(input);
    let mut sum: u64 = 0;
    for i in 0..totals.len() {
        let mut to_observe: Vec<Vec<u64>> = Vec::new();
        to_observe.push(numbers[i].clone());
        'outer: while to_observe.len() > 0 {
            let mut current = to_observe.remove(0);
            if current.len() > 1 {
                let a = current.remove(0);
                let b = current.remove(0);
                for op in OPCODES.iter() {
                    let result = op(a, b);
                    if result == totals[i] {
                        sum += result;
                        break 'outer;
                    }
                    if result < totals[i] && current.len() > 0 {
                        let mut current_clone = current.clone();
                        current_clone.insert(0, result);
                        to_observe.push(current_clone);
                    }
                }
            }
        }
    }
    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (totals, numbers) = parse(input);
    let mut sum: u64 = 0;
    for i in 0..totals.len() {
        let mut to_observe: Vec<Vec<u64>> = Vec::new();
        to_observe.push(numbers[i].clone());
        'outer: while to_observe.len() > 0 {
            let mut current = to_observe.remove(0);
            if current.len() > 1 {
                let a = current.remove(0);
                let b = current.remove(0);
                for op in OPCODES_PART2.iter() {
                    let result = op(a, b);
                    if result == totals[i] {
                        sum += result;
                        break 'outer;
                    }
                    if result < totals[i] && current.len() > 0 {
                        let mut current_clone = current.clone();
                        current_clone.insert(0, result);
                        to_observe.push(current_clone);
                    }
                }
            }
        }
    }
    Some(sum as u64)
}

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<u64>>) {
    let mut totals = Vec::new();
    let mut numbers = Vec::new();
    for item in input.lines() {
        let parts: Vec<&str> = item.split(": ").collect();
        totals.push(parts[0].parse().unwrap());
        let n: Vec<u64> = parts[1].split(" ").map(|x| x.parse().unwrap()).collect();
        numbers.push(n);
    }
    (totals, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
