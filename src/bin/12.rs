use std::collections::HashMap;

advent_of_code::solution!(12);

const DIR: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse(input);

    let mut areas: HashMap<i32, i32> = HashMap::new();
    let mut parm: HashMap<i32, i32> = HashMap::new();

    let mut to_visit: Vec<Vec<i32>> = Vec::new();
    let mut region_id = 0;
    while grid.len() > 0 {
        let item = grid.iter().next().unwrap();
        to_visit.push(vec![item.0 .0, item.0 .1]);
        while to_visit.len() > 0 {
            let node = to_visit.pop().unwrap();
            let mut negbors = 0;
            for (dx, dy) in DIR.iter() {
                let x = node[0] + dx;
                let y = node[1] + dy;
                if grid.contains_key(&(x, y)) && grid.contains_key(&(node[0], node[1]))   {
                    if grid.get(&(x, y)).unwrap() == grid.get(&(node[0], node[1])).unwrap() {
                        negbors += 1;
                        to_visit.push(vec![x, y]);
                    }
                }
            }
            grid.remove(&(node[0], node[1]));
            *areas.entry(region_id).or_insert(1) += 1;
            *parm.entry(region_id).or_insert(0) += 4 - negbors;
        }

        region_id += 1;
    }

    println!("{:?}", areas);
    println!("{:?}", parm);


    let mut sum = 0;
    for i in 0..region_id {
        println!("{},{},{} ", i, areas.get(&i).unwrap(), parm.get(&i).unwrap());
        sum += areas.get(&i).unwrap() * areas.get(&i).unwrap();
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
