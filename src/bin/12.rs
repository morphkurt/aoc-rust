use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    s: i32,
}

#[derive(Debug, Clone)]
struct Region {
    area: i32,
    points: HashSet<(i32,i32)>,

}


const DIR: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn part_one(input: &str) -> Option<u32> {
    let orignal_grid = parse(input);
    let mut grid = orignal_grid.clone();

    let mut areas: HashMap<i32, i32> = HashMap::new();
    let mut parm: HashMap<i32, i32> = HashMap::new();

    let mut to_visit: HashSet<(i32, i32)> = HashSet::new();
    let mut region_id = 0;
    while grid.len() > 0 {
        let item = grid.iter().next().unwrap();
        to_visit.insert((item.0 .0, item.0 .1));
        while to_visit.len() > 0 {
            let node = *to_visit.iter().next().unwrap();
            to_visit.remove(&node);
            let mut neighbors = 4;
            for (dx, dy) in DIR.iter() {
                let x = node.0 + dx;
                let y = node.1 + dy;
                if grid.contains_key(&(x, y)) {
                    if grid.get(&(x, y)).unwrap() == grid.get(&(node.0, node.1)).unwrap() {
                        to_visit.insert((x, y));
                    }
                }
                if orignal_grid.contains_key(&(x, y)) {
                    if orignal_grid.get(&(x, y)).unwrap() == grid.get(&(node.0, node.1)).unwrap() {
                        neighbors -= 1;
                    }
                }
            }
            grid.remove(&(node.0, node.1));
            *areas.entry(region_id).or_insert(0) += 1;
            *parm.entry(region_id).or_insert(0) += neighbors;
        }
        region_id += 1;
    }
    let mut sum = 0;
    for i in 0..region_id {
        sum += areas.get(&i).unwrap() * parm.get(&i).unwrap();
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let orignal_grid = parse(input);
    let mut grid = orignal_grid.clone();

    let mut areas: HashMap<i32, i32> = HashMap::new();
    let mut parm: HashMap<i32, i32> = HashMap::new();

    let mut to_visit: HashSet<(i32, i32)> = HashSet::new();
    let mut region_id = 0;
    while grid.len() > 0 {
        let item = grid.iter().next().unwrap();
        let mut crn: [i32; 4] = [0,0,0,0];
        to_visit.insert((item.0 .0, item.0 .1));
        while to_visit.len() > 0 {
            let node = *to_visit.iter().next().unwrap();
            to_visit.remove(&node);
            let mut neighbors = 4;
            let mut d = 0;
            for (dx, dy) in DIR.iter() {
                let x = node.0 + dx;
                let y = node.1 + dy;
                if grid.contains_key(&(x, y)) {
                    if grid.get(&(x, y)).unwrap() == grid.get(&(node.0, node.1)).unwrap() {
                        to_visit.insert((x, y));
                    }
                }
                if orignal_grid.contains_key(&(x, y)) {
                    if orignal_grid.get(&(x, y)).unwrap() == grid.get(&(node.0, node.1)).unwrap() {
                        neighbors -= 1;
                        crn[d]+=1;
                    }
                }
                d+=1;
            }
            grid.remove(&(node.0, node.1));
            *areas.entry(region_id).or_insert(0) += crn.iter().sum::<i32>();
            *parm.entry(region_id).or_insert(0) += neighbors;
        }
        region_id += 1;
    }
    let mut sum = 0;
    for i in 0..region_id {
        sum += areas.get(&i).unwrap() * parm.get(&i).unwrap();
    }
    Some(sum as u32)
}

fn search_region(id: i32, region_cluster: HashMap<i32,Region>) -> i32 {
    let region = region_cluster.get(&id).unwrap();
    const SEARCH_SPACE: [(i32, i32); 9] =  [(-1,-1),(0,-1),(1,1),(-1,0),(0,0),(1,0),(1,-1),(1,0),(1,1)];

    //find bounding box

    let max_x= region.points.iter().map(|f|f.0).max().unwrap();
    let max_y= region.points.iter().map(|f: &(i32, i32)|f.1).max().unwrap();
    let min_x= region.points.iter().map(|f: &(i32, i32)|f.0).min().unwrap();
    let min_y= region.points.iter().map(|f: &(i32, i32)|f.1).min().unwrap();
    let mut corners= 0;
    for x in -10..20 {
        for y in -10..20 {
            let mut items = 0;
            for s in SEARCH_SPACE  {
                let dx = x + s.0;
                let dy = y + s.1;
                if region.points.contains(&(dx,dy)) {
                    items+=1;
                }
            }
            if items == 5 || items == 4 {
                corners+=1;
            }
        }
    }
    corners
}

fn get_region_id(point: (i32,i32), map: HashMap<i32,Region>) -> i32 {
    for (key, values) in map.iter() {
        if values.points.contains(&point) {
            return *key;
        }
    }
    return -1;
}

fn parse(input: &str) -> HashMap<(i32, i32), char> {
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
