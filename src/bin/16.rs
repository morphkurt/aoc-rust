use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    i32, usize,
};

advent_of_code::solution!(16);

#[derive(Debug, Eq, Clone)]
struct Node {
    loc: (i32, i32),
    cost: usize,
    dir: i32,
    route: HashSet<(i32, i32)>,
}

// Implement PartialEq for Node
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

// Implement Ord for Node
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the comparison to turn the max-heap into a min-heap
        other.cost.cmp(&self.cost)
    }
}

// Implement PartialOrd for Node
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIR: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)]; //R , U , L , D

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start, end) = parse(input); // Assume `parse` gives the grid, start, and end points

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut distances: HashMap<(i32, i32), u32> = HashMap::new(); // Tracks shortest distance to each node
    let mut visited: HashSet<(i32, i32)> = HashSet::new(); // Tracks visited nodes

    // Initialize with the start node
    heap.push(Node {
        loc: start,
        cost: 0,
        dir: 0,
        route: HashSet::new(),
    });
    distances.insert(start, 0);

    while let Some(node) = heap.pop() {
        // Skip already visited nodes
        if visited.contains(&node.loc) {
            continue;
        }
        visited.insert(node.loc);

        // If the end node is reached, return the cost
        if node.loc == end {
            return Some(node.cost as u32);
        }

        // Explore neighbors
        for (i, &(dx, dy)) in DIR.iter().enumerate() {
            let neighbor = (node.loc.0 + dx, node.loc.1 + dy);

            if let Some(c) = grid.get(&neighbor) {
                if visited.contains(&neighbor) {
                    continue;
                }

                // Calculate movement cost based on the given rotation logic
                let rotation_cost = (node.dir - i as i32).abs();
                let cost = match rotation_cost {
                    0 => 1,
                    2 => 2001,
                    1 | 3 => 1001,
                    _ => 0,
                };

                if c == &'.' || c == &'E' {
                    let mut route = node.route.clone();
                    route.insert(node.loc);
                    let new_cost = (node.cost + cost) as u32;
                    if new_cost < *distances.get(&neighbor).unwrap_or(&u32::MAX) {
                        distances.insert(neighbor, new_cost);
                        heap.push(Node {
                            loc: neighbor,
                            cost: new_cost as usize,
                            dir: i as i32,
                            route: route,
                        });
                    }
                }
            }
        }
    }
    None
    // Some(*min as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start, end) = parse(input); // Assume `parse` gives the grid, start, and end points

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut distances: HashMap<(i32, i32, i32), u32> = HashMap::new(); // Tracks shortest distance to each node
    let mut seats: HashSet<(i32, i32)> = HashSet::new();

    let mut min: i32 = i32::MAX;

    // Initialize with the start node
    heap.push(Node {
        loc: start,
        cost: 0,
        dir: 0,
        route: HashSet::new(),
    });
    distances.insert((start.0, start.1, 0), 0);

    while let Some(node) = heap.pop() {
        // Skip already visited nodes
        // If the end node is reached, return the cost
        if node.loc == end {
            min = node.cost as i32;
            seats.extend(node.route.clone());
        }

        if node.cost > min as usize {
            continue;
        }

        // Explore neighbors
        for (i, &(dx, dy)) in DIR.iter().enumerate() {
            let neighbor = (node.loc.0 + dx, node.loc.1 + dy);

            if let Some(c) = grid.get(&neighbor) {
                // Calculate movement cost based on the given rotation logic
                let rotation_cost = (node.dir - i as i32).abs();
                let cost = match rotation_cost {
                    0 => 1,
                    2 => 2001,
                    1 | 3 => 1001,
                    _ => 0,
                };

                if c == &'.' || c == &'E' {
                    let mut route = node.route.clone();
                    route.insert(node.loc);
                    let new_cost = (node.cost + cost) as u32;
                    if new_cost
                        <= *distances
                            .get(&(neighbor.0, neighbor.1, i as i32))
                            .unwrap_or(&u32::MAX)
                    {
                        distances.insert((neighbor.0, neighbor.1, i as i32), new_cost);
                        heap.push(Node {
                            loc: neighbor,
                            cost: new_cost as usize,
                            dir: i as i32,
                            route: route,
                        });
                    }
                }
            }
        }
    }
    Some(seats.len() as u32 + 1)
    // Some(*min as u32)
}

pub fn parse(input: &str) -> (HashMap<(i32, i32), char>, (i32, i32), (i32, i32)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    grid.insert((x as i32, y as i32), c);
                    start = (x as i32, y as i32);
                }
                'E' => {
                    grid.insert((x as i32, y as i32), c);
                    end = (x as i32, y as i32);
                }
                '#' | '.' => {
                    grid.insert((x as i32, y as i32), c);
                }
                _ => {}
            }
        }
    }
    (grid, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
