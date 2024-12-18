use std::{borrow::BorrowMut, collections::HashMap, iter, rc::Rc};


advent_of_code::solution!(15);

#[derive(Debug, Clone)]
struct Box {
    pos: Vec<Vec<i32>>,
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Vec<i32>,
}

#[derive(Debug, Clone)]
struct Fence {
    pos: Vec<i32>,
}

#[derive(Debug, Clone)]
struct Empty {}

trait Movable {
    fn move_object(&mut self, dir: i32);
    fn get_loc(&self) -> Vec<Vec<i32>>;
}

impl Movable for Box {
    fn move_object(&mut self, dir: i32) {
        self.pos[0][0] = self.pos[0][0] + DIR[dir as usize].0;
        self.pos[0][1] = self.pos[0][1] + DIR[dir as usize].1;
        self.pos[1][0] = self.pos[1][0] + DIR[dir as usize].0;
        self.pos[1][1] = self.pos[1][1] + DIR[dir as usize].1;
    }
    fn get_loc(&self) -> Vec<Vec<i32>>{
        self.pos.clone()
    }
}

impl Movable for Robot {
    fn move_object(&mut self, dir: i32) {
        self.pos[0] = self.pos[0] + DIR[dir as usize].0;
        self.pos[1] = self.pos[1] + DIR[dir as usize].1;
    }
    fn get_loc(&self) -> Vec<Vec<i32>>{
        vec![self.pos.clone()]
    }
}

impl Movable for Fence {
    fn move_object(&mut self, _: i32) {}
    fn get_loc(&self) -> Vec<Vec<i32>>{
        vec![self.pos.clone()]
    }
}

#[derive(Debug, Clone)]
enum Obstacle {
    Box(Box),
    Robot(Robot),
    Fence(Fence),
    Empty(Empty),
}

const DIR: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, dir, _) = parse(input);
    let mut temp_grid = grid.clone();

    for direction in dir {
        temp_grid = move_robot(direction, temp_grid.clone());
    }
    let sum: i32 = temp_grid
        .iter()
        .filter(|(_, v)| v == &&'O')
        .map(|(k, _)| k.0 + k.1 * 100)
        .sum();
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, dir,w,h) = parse_pt2(input);
    print_grid_pt2(grid, w, h);
    None
}

fn move_robot_pt2(d:i32, grid: Vec<Obstacle>, w: i32, h: i32) -> Vec<Obstacle> {
    let current_items: Vec<Obstacle> = grid
    .iter()
    .filter_map(|o| {
        if let Obstacle::Robot(robot_obj) = o {
            Some(Obstacle::Robot(o))
        } else {
            None
        }
    }).collect();

    let mut loc = robot.get_loc();

    let mut working_grid = grid.clone();

    loop {
        let next_items: Vec<Option<Obstacle>> = current_items.iter().flat_map(|f|{
            let mut nx = 0;
            let mut ny = 0;
            match f {
                Obstacle::Box(f) => {
                    let p = f.get_loc();
                    if d == 0 || d == 2 {
                        vec![get_obstacle(grid, p[0]+DIR[d as usize].0, p[0]+DIR[d as usize].0, w, h), get_obstacle(grid, p[1]+DIR[d as usize].0, p[1]+DIR[d as usize].0, w, h)];
                    }
                    else if d == 1{
                        re vec![get_obstacle(grid, p[1]+DIR[d as usize].0, p[1]+DIR[d as usize].0, w, h)];
                    }
                        vec![get_obstacle(grid, p[0]+DIR[d as usize].0, p[0]+DIR[d as usize].0, w, h)]

                }
                Obstacle::Robot(robot) => todo!(),
                Obstacle::Fence(fence) => todo!(),
                Obstacle::Empty(empty) => todo!(),
            }
        }
        None
    ).collect();
    }
}

fn move_robot(d: i32, grid: HashMap<(i32, i32), char>) -> HashMap<(i32, i32), char> {
    let mut current: (i32, i32) = grid
        .iter()
        .filter(|(_, v)| v == &&'@')
        .map(|(k, _)| k.clone())
        .next()
        .unwrap();
    let mut staged: Vec<(i32, i32, char)> = Vec::new();
    let mut working_grid = grid.clone();
    staged.push((current.0, current.1, '.'));

    loop {
        let next = (current.0 + DIR[d as usize].0, current.1 + DIR[d as usize].1);
        let c = grid.get(&next);
        if c.is_none() {
            break;
        }
        match c.unwrap() {
            '.' => {
                staged.push((next.0, next.1, *grid.get(&current).unwrap()));
                staged.iter().for_each(|f| {
                    working_grid.insert((f.0, f.1), f.2);
                });
                break;
            }
            'O' => {
                staged.push((next.0, next.1, *grid.get(&current).unwrap()));
            }
            _ => break,
        }
        current = next;
    }
    working_grid
}

fn parse(input: &str) -> (HashMap<(i32, i32), char>, Vec<i32>, (i32, i32)) {
    let s: Vec<&str> = input.trim().split("\n\n").collect();
    let mut o: (i32, i32) = (0, 0);
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in s[0].lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                o = (x as i32, y as i32)
            }
            map.insert((x as i32, y as i32), c);
        }
    }
    let dir: Vec<i32> = s[1]
        .replace("\n", "")
        .chars()
        .map(|c| {
            return match c {
                '^' => 0,
                '>' => 1,
                'v' => 2,
                '<' => 3,
                _ => 0,
            };
        })
        .collect();
    (map, dir, o)
}

fn parse_pt2(input: &str) -> (Vec<Obstacle>, Vec<i32>,i32,i32) {
    let s: Vec<&str> = input.trim().split("\n\n").collect();
    let mut map: Vec<Obstacle> = Vec::new();
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    for (y, line) in s[0].lines().enumerate() {
        h = y as i32;
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    let b = Box {
                        pos: vec![
                            vec![(x * 2) as i32, y as i32],
                            vec![(x * 2) as i32 + 1, y as i32],
                        ],
                    };
                    map.push(Obstacle::Box(b));
                }
                '@' => {
                    map.push(Obstacle::Robot(Robot {
                        pos: vec![(x * 2) as i32, y as i32],
                    }));
                }
                '#' => {
                    let f1 = Fence {
                        pos: vec![(x * 2) as i32, y as i32],
                    };
                    let f2 = Fence {
                        pos: vec![((x * 2) + 1) as i32, y as i32],
                    };
                    map.push(Obstacle::Fence(f1));
                    map.push(Obstacle::Fence(f2));
                }
                _ => (),
            }
            w = (x*2) as i32 + 1;
        }
    }
    let dir: Vec<i32> = s[1]
        .replace("\n", "")
        .chars()
        .map(|c| {
            return match c {
                '^' => 0,
                '>' => 1,
                'v' => 2,
                '<' => 3,
                _ => 0,
            };
        })
        .collect();
    (map, dir, w+1, h+1)
}

fn _print_grid(grid: &HashMap<(i32, i32), char>, w: i32, h: i32) {
    for y in 0..h {
        for x in 0..w {
            print!("{}", grid.get(&(x, y)).unwrap())
        }
        println!("");
    }
}

fn print_grid_pt2(grid: Vec<Obstacle>, w: i32, h: i32) {
    for y in 0..h {
        for x in 0..w {
            let o = get_obstacle(grid.clone(), x, y, w, h).unwrap();
            match o {
                Obstacle::Box(o) => {
                    if o.pos[0][0] == x && o.pos[0][1] == y {
                        print!("[");
                    }
                    if o.pos[1][0] == x && o.pos[1][1] == y {
                        print!("]");
                    }
                }
                Obstacle::Robot(_) => {
                    print!("@");
                }
                Obstacle::Fence(_) => {
                    print!("#");
                }
                Obstacle::Empty(_) => {
                    print!(".");
                }
            }
        }
        println!("");
    }
}

fn get_obstacle(grid: Vec<Obstacle>, x: i32, y: i32, w: i32, h: i32) -> Option<Obstacle> {
    if x > w || x < 0 || y > h || y < 0 {
        return None;
    }
    for o in grid {
        match o {
            Obstacle::Box(o) => {
                if o.pos[0][0] == x && o.pos[0][1] == y || o.pos[1][0] == x && o.pos[1][1] == y {
                    return Some(Obstacle::Box(o));
                }
            }
            Obstacle::Robot(o) => {
                if o.pos[0] == x && o.pos[1] == y {
                    return Some(Obstacle::Robot(o));
                }
            }
            Obstacle::Fence(o) => {
                if o.pos[0] == x && o.pos[1] == y {
                    return Some(Obstacle::Fence(o));
                }
            }
            Obstacle::Empty(_) => return None,
        }
    }
    return Some(Obstacle::Empty(Empty {}));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
