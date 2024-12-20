use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

advent_of_code::solution!(20);

#[derive(Debug, PartialEq, Eq, Clone)]
enum SpaceType {
    OUTERWALL(),
    INNER(),
    EMPTY(),
}

const DIR: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)]; //R , U , L , D

#[derive(Debug, Clone)]
struct Space {
    loc: (i32, i32),
    space_type: SpaceType,
}

#[derive(Debug, Eq, Clone)]
struct Node {
    loc: (i32, i32),
    visited: Vec<(i32, i32)>,
}

// Implement PartialEq for Node
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.visited.len() == other.visited.len()
    }
}

// Implement Ord for Node
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the comparison to turn the max-heap into a min-heap
        other.visited.len().cmp(&self.visited.len())
    }
}

// Implement PartialOrd for Node
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start, end) = parse(input);

    let (_, best_route) = walk(grid.clone(), start, end);

    let mut sum = 0;

    for i in 0..best_route.len() {
        for k in 0..best_route.len() {
            let left = best_route[i];
            let right = best_route[k];
            let dx = (left.0 - right.0).abs();
            let dy = (left.1 - right.1).abs();
            if (k as i32 - i as i32) - (dx + dy) >= 100 && (dx + dy) <= 2 && (dx + dy) > 1 {
                sum += 1;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start, end) = parse(input);
    let (_, best_route) = walk(grid.clone(), start, end);

    let mut sum = 0;

    for i in 0..best_route.len() {
        for k in 0..best_route.len() {
            let left = best_route[i];
            let right = best_route[k];

            let dx = (left.0 - right.0).abs();
            let dy = (left.1 - right.1).abs();
            if (k as i32 - i as i32) - (dx + dy) >= 100 && (dx + dy) <= 20 && (dx + dy) > 1 {
                sum += 1;
            }
        }
    }
    Some(sum)
}

fn walk(
    grid: HashMap<(i32, i32), Space>,
    start: (i32, i32),
    end: (i32, i32),
) -> (i32, Vec<(i32, i32)>) {
    let mut to_visit: BinaryHeap<Node> = BinaryHeap::new();
    let mut distance: HashMap<(i32, i32), usize> = HashMap::new();
    to_visit.push(Node {
        loc: start,
        visited: Vec::new(),
    });

    loop {
        if to_visit.is_empty() {
            break;
        }
        distance.insert(start, 0);
        let node = to_visit.pop().unwrap();
        if node.loc == end {
            let mut final_route = node.visited.clone();
            final_route.push(node.loc);
            return (node.visited.len() as i32, final_route);
        }
        for dir in DIR {
            if grid.contains_key(&(node.loc.0 + dir.0, node.loc.1 + dir.1)) {
                if grid
                    .get(&(node.loc.0 + dir.0, node.loc.1 + dir.1))
                    .unwrap()
                    .space_type
                    == SpaceType::EMPTY()
                {
                    let next = grid.get(&(node.loc.0 + dir.0, node.loc.1 + dir.1)).unwrap();
                    if node.visited.contains(&next.loc) {
                        continue;
                    }

                    let new_cost = node.visited.len() + 1;
                    let mut visited = node.visited.clone();
                    visited.push(node.loc);
                    if new_cost < *distance.get(&next.loc).unwrap_or(&usize::MAX) {
                        distance.insert(next.loc, new_cost);
                        to_visit.push(Node {
                            loc: (node.loc.0 + dir.0, node.loc.1 + dir.1),
                            visited,
                        });
                    }
                }
            }
        }
    }
    return (0, Vec::new());
}

fn parse(input: &str) -> (HashMap<(i32, i32), Space>, (i32, i32), (i32, i32)) {
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    let mut grid: HashMap<(i32, i32), Space> = HashMap::new();
    let mut y: i32 = 0;
    for rows in input.lines() {
        let mut x: i32 = 0;
        for c in rows.chars() {
            match c {
                'S' => start = (x, y),
                'E' => end = (x, y),
                _ => (),
            }
            let space_type: SpaceType = match c {
                '#' => {
                    if x == 0 || y == 0 {
                        SpaceType::OUTERWALL();
                    }
                    SpaceType::INNER()
                }
                _ => SpaceType::EMPTY(),
            };
            grid.insert(
                (x, y),
                Space {
                    loc: (x, y),
                    space_type,
                },
            );
            x += 1;
        }
        y += 1;
    }
    (grid, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(84));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
