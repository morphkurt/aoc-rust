use std::{cmp::max, collections::HashSet};

use rayon::str;

advent_of_code::solution!(8);

#[derive(Debug, Clone)]
struct Grid {
    w: i32,
    h: i32,
    points: HashSet<Point>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
    c: char,
}

fn find_anti_nodes_part1(a: &Point, b: &Point) -> Vec<Point> {
    let mut a1 = a;
    let mut b1 = b;
    let mut nodes: Vec<Point> = Vec::new();
    let d_x = (a.x - b.x).abs();
    let d_y = (a.y - b.y).abs();

    if a.x > b.x {
        let temp_a = a;
        let temp_b = b;
        b1 = temp_a;
        a1 = temp_b;
    }

    if a1.y >= b1.y {
        nodes.push(Point {
            x: b1.x + d_x,
            y: b1.y - d_y,
            c: a.c,
        });
        nodes.push(Point {
            x: a1.x - d_x,
            y: a1.y + d_y,
            c: a.c,
        });
    } else if a1.y <= b1.y {
        nodes.push(Point {
            x: b1.x + d_x,
            y: b1.y + d_y,
            c: a.c,
        });
        nodes.push(Point {
            x: a1.x - d_x,
            y: a1.y - d_y,
            c: a.c,
        });
    }
    nodes
}

fn find_anti_nodes_part2(a: &Point, b: &Point, w: i32, h: i32) -> Vec<Point> {
    let mut nodes: Vec<Point> = Vec::new();
    let d_x = (a.x - b.x).abs();
    let d_y = (a.y - b.y).abs();

    let max_x = w / d_x;
    let max_y = h / d_y;

    let max = max(max_x, max_y);

    let (a1, b1) = if a.x > b.x { (b, a) } else { (a, b) };

    for i in 0..max {
        if a1.y >= b1.y {
            nodes.push(Point {
                x: b1.x + d_x * i,
                y: b1.y - d_y * i,
                c: a1.c,
            });
            nodes.push(Point {
                x: a1.x - d_x * i,
                y: a1.y + d_y * i,
                c: a1.c,
            });
        } else if a1.y <= b1.y {
            nodes.push(Point {
                x: b1.x + d_x * i,
                y: b1.y + d_y * i,
                c: a1.c,
            });
            nodes.push(Point {
                x: a1.x - d_x * i,
                y: a1.y - d_y * i,
                c: a1.c,
            });
        }
    }
    nodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let unique_antennas = grid.points.iter().map(|x| x.c).collect::<HashSet<char>>();
    let mut anitnodes: Vec<Point> = Vec::new();
    for antenna_type in unique_antennas.iter() {
        let antennas = grid
            .points
            .iter()
            .filter(|x| x.c == *antenna_type)
            .collect::<Vec<&Point>>();
        for i in 0..antennas.len() {
            for j in 0..antennas.len() {
                let a = antennas[i];
                let b = antennas[j];
                if i == j {
                    continue;
                }
                let nodes = find_anti_nodes_part1(a, b);
                anitnodes.extend(nodes);
            }
        }
    }
    anitnodes.retain(|x| x.x >= 0 && x.y >= 0 && x.x < grid.w && x.y < grid.h);
    let uniq = anitnodes
        .iter()
        .map(|x| format!("{}-{}", x.x, x.y))
        .collect::<HashSet<String>>();
    Some(uniq.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    let unique_antennas = grid.points.iter().map(|x| x.c).collect::<HashSet<char>>();
    let mut anitnodes: Vec<Point> = Vec::new();
    for antenna_type in unique_antennas.iter() {
        let antennas = grid
            .points
            .iter()
            .filter(|x| x.c == *antenna_type)
            .collect::<Vec<&Point>>();
        for i in 0..antennas.len() {
            for j in 0..antennas.len() {
                let a = antennas[i];
                let b = antennas[j];
                if i == j {
                    continue;
                }
                let nodes = find_anti_nodes_part2(a, b, grid.w, grid.h);
                anitnodes.extend(nodes);
            }
        }
    }
    anitnodes.retain(|x| x.x >= 0 && x.y >= 0 && x.x < grid.w && x.y < grid.h);
    let uniq = anitnodes
        .iter()
        .map(|x| format!("{}-{}", x.x, x.y))
        .collect::<HashSet<String>>();
    Some(uniq.len() as u32)
}

fn parse(input: &str) -> Grid {
    let mut w = 0;
    let mut h = 0;
    let mut points: HashSet<Point> = HashSet::new();
    for (y, line) in input.split('\n').into_iter().enumerate() {
        h = y as i32;
        for (x, c) in line.chars().enumerate() {
            w = x as i32;
            if c != '.' {
                points.insert(Point {
                    x: x as i32,
                    y: y as i32,
                    c,
                });
            }
        }
    }
    h += 1;
    w += 1;
    Grid { w, h, points }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
