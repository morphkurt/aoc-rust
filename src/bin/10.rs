use std::collections::HashSet;

advent_of_code::solution!(10);

static UP: [i32; 2] = [0, -1];
static DOWN: [i32; 2] = [0, 1];
static LEFT: [i32; 2] = [-1, 0];
static RIGHT: [i32; 2] = [1, 0];
static DIRECTIONS: [[i32; 2]; 4] = [UP, RIGHT, DOWN, LEFT];

struct Node {
    x: i32,
    y: i32,
    visited: HashSet<(i32, i32)>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, starts) = parse(input);
    let mut sum = 0;
    for start in starts.iter() {
        sum += walk(grid.clone(), start.clone());
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, starts) = parse(input);
    let mut sum = 0;
    for start in starts.iter() {
        sum += walk_part2(grid.clone(), start.clone());
    }

    Some(sum as u32)
}

fn walk(grid: Vec<Vec<i32>>, start: Vec<i32>) -> i32 {
    let mut found_dest: HashSet<(i32, i32)> = HashSet::new();
    let mut to_visit = Vec::new();
    to_visit.push(Node {
        x: start[0],
        y: start[1],
        visited: HashSet::new(),
    });
    let mut score = 0;
    while to_visit.len() > 0 {
        let node = to_visit.pop().unwrap();
        let node_val = grid[node.y as usize][node.x as usize];
        for direction in DIRECTIONS.iter() {
            let x = node.x + direction[0];
            let y = node.y + direction[1];
            if x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32 {
                let value = grid[y as usize][x as usize];
                if node_val + 1 == value {
                    if !node.visited.contains(&(x, y)) {
                        let mut node_visited: HashSet<(i32, i32)> = node.visited.clone();
                        node_visited.insert((node.x, node.y));
                        if value == 9 {
                            if !found_dest.contains(&(x, y)) {
                                found_dest.insert((x, y));
                                score += 1;
                            }
                        } else {
                            to_visit.push(Node {
                                x: x,
                                y: y,
                                visited: node_visited,
                            });
                        }
                    }
                }
            }
        }
    }
    score
}

fn walk_part2(grid: Vec<Vec<i32>>, start: Vec<i32>) -> i32 {
    let mut to_visit = Vec::new();
    to_visit.push(Node {
        x: start[0],
        y: start[1],
        visited: HashSet::new(),
    });
    let mut score = 0;
    while to_visit.len() > 0 {
        let node = to_visit.pop().unwrap();
        let node_val = grid[node.y as usize][node.x as usize];
        for direction in DIRECTIONS.iter() {
            let x = node.x + direction[0];
            let y = node.y + direction[1];
            if x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32 {
                let value = grid[y as usize][x as usize];
                if node_val + 1 == value {
                    if !node.visited.contains(&(x, y)) {
                        let mut node_visited: HashSet<(i32, i32)> = node.visited.clone();
                        node_visited.insert((node.x, node.y));
                        if value == 9 {
                            score += 1;
                        } else {
                            to_visit.push(Node {
                                x: x,
                                y: y,
                                visited: node_visited,
                            });
                        }
                    }
                }
            }
        }
    }
    score
}

fn parse(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut starts: Vec<Vec<i32>> = Vec::new();
    let mut lines = Vec::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        let mut numbers = Vec::new();
        for number in line.chars() {
            if number == '0' {
                starts.push([x, y].to_vec());
            }
            numbers.push(number.to_digit(10).unwrap() as i32);
            x += 1;
        }
        lines.push(numbers);
        y += 1;
    }
    (lines, starts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
