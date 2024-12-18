
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug, Clone)]
struct Game {
    x_a: f64,
    x_b: f64,
    y_a: f64,
    y_b: f64,
    e_x: f64,
    e_y: f64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let g = parse(input);
    let s: u64 =  g.iter().map(|f|calc(f.clone())).filter(|f|f.len() > 0).map(|f| 3* f[0]+ 1*f[1]).sum();
    Some(s as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn calc(g: Game) -> Vec<u64> {
    let mut r: Vec<u64> = Vec::new();
    let b: f64 = (g.e_y*g.x_a-g.e_x*g.y_a)/(g.y_b*g.x_a-g.x_b*g.y_a);
    let a: f64 =(g.e_x-b*g.x_b)/g.x_a;
    if a.fract() == 0.0 && b.fract() == 0.0 {
        r.push(a as u64);
        r.push(b as u64);
    }
    r
}

fn parse(input: &str) -> Vec<Game> {
    let mut game_result: Vec<Game> = Vec::new();
    for game in input.split("\n\n") {
        let mut result: Vec<Vec<u64>> = Vec::new();

        for line in game.lines() {

            let re = Regex::new(r"[+-]?\d+").unwrap();
            // Extract and collect matches
            let numbers: Vec<u64> = re
                .find_iter(line)
                .filter_map(|mat| mat.as_str().parse::<u64>().ok())
                .collect();

            result.push(numbers);
        }
        game_result.push(Game { x_a: result[0][0] as f64, x_b: result[1][0] as f64, y_a: result[0][1] as f64, y_b: result[1][1] as f64, e_x: (result[2][0]+10000000000000) as f64, e_y: (result[2][1]+10000000000000) as f64 });
    }
    game_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
