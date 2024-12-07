use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

#[derive(Debug, Clone)]
enum Object {
    OBSTACLE,
}

#[derive(Debug, Clone)]
struct Grid {
    map: HashMap<Point, Object>,
    w: i32,
    h: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn go(&self, direction: Point) -> Point {
        Point {
            x: self.x + direction.x,
            y: self.y + direction.y,
        }
    }
}

impl Grid {
    fn bound(&self, point: Point) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }
        if point.x >= self.w || point.y >= self.h {
            return false;
        }
        true
    }
    fn obstacle(&self, point: Point) -> bool {
        match self.map.get(&point) {
            Some(Object::OBSTACLE) => true,
            _ => false,
        }
    }
}

const UP: Point = Point { x: 0, y: -1 };
const RIGHT: Point = Point { x: 1, y: 0 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };

const DIRECTIONS: [Point; 4] = [UP, RIGHT, DOWN, LEFT];

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, mut guard) = parse(input);
    let mut d = 0;
    let mut visited: HashSet<Point> = HashSet::new();
    loop {
        let next = guard.go(DIRECTIONS[d]);
        if !grid.bound(next) {
            break;
        }
        if grid.obstacle(next) {
            d = (d + 1) % 4;
        } else {
            visited.insert(next);
            guard = next;
        }
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, origin_guard) = parse(input);
    let mut d: usize = 0;
    let mut guard = origin_guard;
    let mut visited: HashSet<Point> = HashSet::new();
    loop {
        let next = guard.go(DIRECTIONS[d]);
        if !grid.bound(next) {
            break;
        }
        if grid.obstacle(next) {
            d = (d + 1) % 4;
        } else {
            visited.insert(next);
            guard = next;
        }
    }
    let sum: u32 = visited.par_iter().map(|&point| {
        let mut fresh_grid = grid.clone();
        let mut direct_aware_visited = HashSet::new();
        let mut d: usize = 0;
        let mut start_point = origin_guard;
        fresh_grid.map.insert(point, Object::OBSTACLE);

        loop {
            let next = start_point.go(DIRECTIONS[d]);
            let key = (next.x, next.y, d);
            if !fresh_grid.bound(next) {
                break;
            }
            if direct_aware_visited.contains(&key) {
                return 1;
            }
            if fresh_grid.obstacle(next) {
                d = (d + 1) % 4;
            } else {
                direct_aware_visited.insert(key);
                start_point = next;
            }
        }
        0
    }).sum();
    Some(sum)
}

fn parse(input: &str) -> (Grid, Point) {
    let mut w = 0;
    let mut h = 0;
    let mut origin = Point { x: 0, y: 0 };
    let mut map = HashMap::new();
    for (y, line) in input.split('\n').into_iter().enumerate() {
        h = y as i32;
        for (x, c) in line.chars().enumerate() {
            w = x as i32;
            match c {
                '#' => {
                    map.insert(
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                        Object::OBSTACLE,
                    );
                }
                '^' => {
                    origin = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                }
                _ => {}
            };
        }
    }
    h += 1;
    w += 1;
    (Grid { map, w, h }, origin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
