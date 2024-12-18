use std::{collections::HashSet, i32};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let w = 101;
    let h = 103;
    let robots = parse(input);
    let mut moved = robots.clone();
    for i in 0..100000000 {
        moved = moved
            .into_iter()
            .map(|f| {
                let x1 = (f[0] + f[2]) % (w);
                let y1 = (f[1] + f[3]) % (h);
                let x = if x1 < 0 { x1 + w } else { x1 };
                let y = if y1 < 0 { y1 + h } else { y1 };
                vec![x, y, f[2], f[3]]})
            .collect();
     let set: HashSet<String>= moved.iter().map(|f| format!("{}{}",f[0],f[1])).collect();
     if set.len() == moved.len() {
        println!("{}",i);
        print_matrix_with_point(103,101,set);

     }
    }
    let final_pos: Vec<Vec<i32>> = moved
        .into_iter()
        .map(|f| {
            return vec![f[0], f[1]];
        })
        .collect();

    let q1 = final_pos
        .iter()
        .filter(|f| f[0] < w / 2 && f[1] < h / 2)
        .count();
    let q2 = final_pos
        .iter()
        .filter(|f| f[0] > w / 2 && f[1] < h / 2)
        .count();
    let q3 = final_pos
        .iter()
        .filter(|f| f[0] < w / 2 && f[1] > h / 2)
        .count();
    let q4 = final_pos
        .iter()
        .filter(|f| f[0] > w / 2 && f[1] > h / 2)
        .count();

    Some((q1 * q2 * q3 * q4) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn print_matrix_with_point(x: usize, y: usize, set: HashSet<String>) {
    // Define matrix dimensions
    let rows = 103;
    let cols = 101;


    // Print the matrix
    for row in 0..rows {
        for col in 0..cols {
            if set.contains(&format!("{}{}",col,row)) {
                print!("x"); // Print 'x' for the specified point
            } else {
                print!("."); // Print '.' for all other points
            }
        }
        println!(); // Newline after each row
    }
}


fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            l.replace("p=", "")
                .replace(" v=", ",")
                .split(",")
                .map(|f| f.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
