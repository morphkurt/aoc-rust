advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut matrix, w, h, x, y, _) = parse(input);
    let mut d: usize = 3;
    let n: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1]];
    let mut guard: Vec<i32> = vec![x as i32, y as i32];

    matrix[get_array_loc(w, guard[0], guard[1])] = 'X';
    loop {
        guard[0] += n[d][0];
        guard[1] += n[d][1];
        if !(guard[0] >= 0
            && guard[1] >= 0
            && guard[0] < w.try_into().unwrap()
            && guard[1] < h.try_into().unwrap())
        {
            break;
        }
        if matrix[get_array_loc(w, guard[0], guard[1])] == '#' {
            guard[0] -= n[d][0];
            guard[1] -= n[d][1];
            d = (d + 1) % 4;
            guard[0] += n[d][0];
            guard[1] += n[d][1];
        }
        matrix[get_array_loc(w, guard[0], guard[1])] = 'X';
    }
    let sum = matrix.iter().filter(|&n| *n == 'X').count();
    return Some(sum as u32);
}

fn get_array_loc(w: usize, x: i32, y: i32) -> usize {
    let r: i32 = y * w as i32 + x;
    r as usize
}

pub fn part_two(input: &str) -> Option<u32> {
    let (matrix, w, h, x, y, routes) = parse(input);
    let n: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1]];
    let guard: Vec<i32> = vec![x as i32, y as i32];

    let mut sum = 0;
    let size = w * h;

    for i in 0..size {
        let mut cloned_matrix = matrix.clone();
        let mut cloned_routes = routes.clone();
        let mut d: usize = 3;

        cloned_routes[get_array_loc(w, guard[0], guard[1])].push(d as i32);
        if cloned_matrix[i] != '#' {
            cloned_matrix[i] = '#';
        } else {
            continue;
        }
        let mut g: Vec<i32> = guard.clone();
        loop {
            g[0] += n[d][0];
            g[1] += n[d][1];
            if !(g[0] >= 0
                && g[1] >= 0
                && g[0] < w.try_into().unwrap()
                && g[1] < h.try_into().unwrap())
            {
                break;
            }
            if cloned_routes[get_array_loc(w, g[0], g[1])].contains(&(d as i32)) {
                sum += 1;
                break;
            }
            cloned_routes[get_array_loc(w, g[0], g[1])].push(d as i32);

            if cloned_matrix[get_array_loc(w, g[0], g[1])] == '#' {
                g[0] -= n[d][0];
                g[1] -= n[d][1];
                d = (d + 1) % 4;
            }
        }
    }
    return Some(sum as u32);
}

fn parse(input: &str) -> (Vec<char>, usize, usize, usize, usize, Vec<Vec<i32>>) {
    let mut routes: Vec<Vec<i32>> = Vec::new();
    let mut matrix = Vec::new();
    let mut w = 0;
    let mut h = 0;
    let mut x = 0;
    let mut y = 0;
    let mut j = 0;
    for line in input.lines() {
        let mut i = 0;
        for c in line.chars() {
            if c == '^' {
                x = i;
                y = j;
            }
            routes.push(Vec::<i32>::new());
            matrix.push(c);
            i += 1;
        }
        w = line.len();
        h += 1;
        j += 1;
    }
    (matrix, w, h, x, y, routes)
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
