#[derive(Debug, PartialEq)]
struct Point {
    c: Option<char>,
    x: i32,
    y: i32,
}

const NEIGHBORS: &[Point] = &[
    Point {
        c: None,
        x: 0,
        y: -1,
    }, // N 0
    Point {
        c: None,
        x: 0,
        y: 1,
    }, // S 1
    Point {
        c: None,
        x: 1,
        y: 0,
    }, // E 2
    Point {
        c: None,
        x: -1,
        y: 0,
    }, // W 3
    Point {
        c: None,
        x: 1,
        y: -1,
    }, // NE 4
    Point {
        c: None,
        x: -1,
        y: -1,
    }, // NW 5
    Point {
        c: None,
        x: 1,
        y: 1,
    }, // SE 6
    Point {
        c: None,
        x: -1,
        y: 1,
    }, //  SW 7
];

const CROSS: &[Point] = &[
    Point {
        c: None,
        x: 1,
        y: -1,
    }, // NE 4
    Point {
        c: None,
        x: -1,
        y: -1,
    }, // NW 5
    Point {
        c: None,
        x: 1,
        y: 1,
    }, // SE 6
    Point {
        c: None,
        x: -1,
        y: 1,
    }, //  SW 7
];

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let word: &[char] = &['X', 'M', 'A', 'S'];
    let (points, width, height) = parse(input);
    let mut result = 0;
    for p in &points {
        let directions = find_dir(word, p, &points, width, height);
        for d in directions {
            if find(word, 0, p, d, &points, width, height) {
                result += 1;
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (points, width, height) = parse(input);
    let mut result = 0;
    for p in &points {
        if p.c == Some('A') {
            let available = CROSS
                .iter()
                .filter(|n| {
                    let x = p.x + n.x;
                    let y = p.y + n.y;
                    x >= 0
                        && x < width.try_into().unwrap()
                        && y >= 0
                        && y < height.try_into().unwrap()
                })
                .map(|n| {
                    let x = p.x + n.x;
                    let y = p.y + n.y;
                    get_point(&points, x as usize, y as usize, width)
                })
                .collect::<Vec<&Point>>();
            if available.len() == 4 {
                if (available[0].c == Some('S') && available[3].c == Some('M')
                    || available[0].c == Some('M') && available[3].c == Some('S'))
                    && (available[1].c == Some('S') && available[2].c == Some('M')
                        || available[1].c == Some('M') && available[2].c == Some('S'))
                {
                    result += 1;
                }
            }
        }
    }
    Some(result)
}

fn parse(input: &str) -> (Vec<Point>, usize, usize) {
    let points: Vec<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| Point {
                c: Some(c),
                x: x as i32,
                y: y as i32,
            })
        })
        .collect();
    let width = points.iter().max_by_key(|p| p.x).unwrap().x as usize + 1;
    let height = points.iter().max_by_key(|p| p.y).unwrap().y as usize + 1;
    (points, width, height)
}

fn get_point(points: &Vec<Point>, x: usize, y: usize, w: usize) -> &Point {
    &points[(y * w) + x]
}

fn find_dir(
    word: &[char],
    p: &Point,
    points: &Vec<Point>,
    width: usize,
    height: usize,
) -> Vec<usize> {
    let mut result = Vec::new();
    if word[0] == p.c.unwrap() {
        for (i, n) in NEIGHBORS.iter().enumerate() {
            let x = p.x + n.x;
            let y = p.y + n.y;
            if x >= 0 && x < width.try_into().unwrap() && y >= 0 && y < height.try_into().unwrap() {
                let neighbor = get_point(&points, x as usize, y as usize, width);
                if neighbor.c == Some(word[1]) {
                    result.push(i);
                }
            }
        }
    }
    result
}

fn find(
    word: &[char],
    index: usize,
    p: &Point,
    d: usize,
    points: &Vec<Point>,
    width: usize,
    height: usize,
) -> bool {
    if index == word.len() {
        return true;
    }
    if p.c.unwrap() == word[index] {
        if (index + 1) == word.len() {
            return true;
        }
        let n = &NEIGHBORS[d];
        let x = p.x + n.x;
        let y = p.y + n.y;
        if x >= 0 && x < width.try_into().unwrap() && y >= 0 && y < height.try_into().unwrap() {
            let neighbor = get_point(&points, x as usize, y as usize, width);
            return find(word, index + 1, neighbor, d, points, width, height);
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
