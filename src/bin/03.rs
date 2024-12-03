use regex::Regex;
use scanf::sscanf;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(?m)mul\(\d+,\d+\)").unwrap();
    let result = regex.captures_iter(input);

    let sum: i32 = result.into_iter().map(|mat| {
        let mut a = 0;
        let mut b = 0;
        let s : &str = mat.get(0).unwrap().as_str();
        let _ = sscanf!(s, "mul({},{})", a, b);
        a * b
    }).sum();
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(?m)mul\(\d+,\d+\)|don't|do").unwrap();
    let result = regex.captures_iter(input);

    let mut coef = 1;
    let mut sum = 0;

    for mat in result {
        let s: &str = mat.get(0).unwrap().as_str();
        match s {
            s if s.contains("don't") => coef = 0,
            s if s.contains("do") => coef = 1,
            s if s.contains("mul") => {
                let mut a = 0;
                let mut b = 0;
                let _ = sscanf!(s, "mul({},{})", a, b);
                sum += coef * a * b;
            }
            _ => {}
        }
    }
    Some(sum as u32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
