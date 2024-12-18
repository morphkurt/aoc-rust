use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

advent_of_code::solution!(18);

const DIR: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)]; //R , U , L , D


#[derive(Debug, Eq, Clone)]
struct Node {
    loc: (i32, i32),
    cost: usize,
    route: Vec<(i32, i32)>,
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

pub fn part_one(input: &str) -> Option<u32> {
    let all: Vec<(i32,i32)> = parse(input);
    let end: (i32, i32) = (70,70);

    Some(walk(all, 1024, end) as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let all: Vec<(i32,i32)> = parse(input);

    let end: (i32, i32) = (70,70);

    let mut low = 0;
    let mut high = all.len();


    while low <= high {
        let mid = (low + high) / 2;

        if walk(all.clone(), mid, end)  > 0 {
            low = mid + 1; // Move to the right half
        } else {
            if mid == 0 || walk(all.clone(), mid-1, end) > 0 {
                return Some(format!("{},{}",all[mid-1].0, all[mid-1].1)); // Found the first false value
            }
            high = mid - 1; // Move to the left half
        }
    }
    None
}

fn walk(all: Vec<(i32,i32)>, index: usize, end: (i32,i32)) -> usize {
    let dimension = end.0;
    let grid = &all[..index.min(all.len())];
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut distances: HashMap<(i32, i32), u32> = HashMap::new();

    heap.push(Node {
        loc: (0,0),
        cost: 0,
        route: Vec::new(),
    });

    while let Some(node) = heap.pop() {
        if node.loc == end {
            return node.cost;
        }
        for (_, &(dx, dy)) in DIR.iter().enumerate() {
            let neighbor = (node.loc.0 + dx, node.loc.1 + dy);

            if neighbor.0  < 0 || neighbor.0  > dimension || neighbor.1 < 0 || neighbor.1 > dimension {
                continue;
            }

            if node.route.contains(&neighbor) {
                continue;
            }

            if grid.contains(&neighbor) {
                continue;
            }

            let mut route = node.route.clone();
            route.push(node.loc);
            let new_cost = (node.cost + 1) as u32;
            if new_cost < *distances.get(&neighbor).unwrap_or(&u32::MAX) {
                distances.insert(neighbor, new_cost);
                heap.push(Node {
                    loc: neighbor,
                    cost: new_cost as usize,
                    route: route,
                });
            }
        }
    }
    0
}

fn parse(input: &str) -> Vec<(i32,i32)> {
    input.lines()
    .map(|f|{
        let s = f.split(",").collect::<Vec<&str>>();
        (s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap())
    }).collect::<Vec<(i32,i32)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("(6,1)".to_string()));
    }
}
