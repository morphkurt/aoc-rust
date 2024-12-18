use std::{char::MAX, result};

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator, StepBy};

advent_of_code::solution!(17);

const OPCODES: [fn(Vec<u64>, u64) -> (Vec<u64>, Option<u64>); 8] =
    [adv, bxl, bst, jnz, bxc, out, bdv, cdv];

fn map(r: Vec<u64>, operand: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => r[0],
        5 => r[1],
        6 => r[2],
        _ => operand,
    }
}

fn bst(r: Vec<u64>, operand: u64) -> (Vec<u64>, Option<u64>) {
    let mut reg: Vec<u64> = r.clone();
    let v = map(r, operand);
    reg[1] = v % 8;
    (reg, None)
}

fn out(r: Vec<u64>, operand: u64) -> (Vec<u64>, Option<u64>) {
    let v = map(r.clone(), operand);
    (r, Some(v % 8))
}

fn jnz(r: Vec<u64>, _operand: u64) -> (Vec<u64>, Option<u64>) {
    (r, None)
}

fn bdv(r: Vec<u64>, operand: u64) -> (Vec<u64>, Option<u64>) {
    let mut reg = r.clone();
    let v = map(r, operand);
    reg[1] = reg[0] / ((2 as f64).powf(v as f64) as u64);
    (reg, None)
}

fn cdv(r: Vec<u64>, operand: u64) -> (Vec<u64>, Option<u64>) {
    let mut reg = r.clone();
    let v = map(r, operand);
    reg[2] = reg[0] / ((2 as f64).powf(v as f64) as u64);
    (reg, None)
}

fn adv(r: Vec<u64>, operand: u64) -> (Vec<u64>, Option<u64>) {
    let mut reg = r.clone();
    let v = map(r, operand);
    reg[0] = reg[0] / ((2 as f64).powf(v as f64) as u64);
    (reg, None)
}

fn bxl(mut r: Vec<u64>, operand: u64) -> (Vec<u64>, Option<u64>) {
    r[1] = r[1] ^ operand;
    (r, None)
}

fn bxc(mut r: Vec<u64>, _operand: u64) -> (Vec<u64>, Option<u64>) {
    r[1] = r[1] ^ r[2];
    (r, None)
}

pub fn part_one(input: &str) -> Option<String> {
    let (r, p) = parse(input);

    let mut temp_r = r.clone();
    let mut out: Vec<u64> = Vec::new();
    let mut pointer: usize = 0;
    loop {
        if pointer >= p.len() {
            break;
        }
        if p[pointer][0] == 3 && temp_r[0] != 0 {
            pointer = p[pointer][1] as usize;
        } else {
            let (r_r, s) = OPCODES[p[pointer][0] as usize](temp_r.clone(), p[pointer][1]);
            temp_r = r_r;
            if s.is_some() {
                out.push(s.unwrap());
            }
            pointer += 1;
        }
    }
    Some(
        out.iter()
            .into_iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (r, p) = parse(input);


//    Use parallel iterator to search for the correct 'v'
    let result: Vec<Vec<u64>> = (165323019388784..165363019388784)
        .into_iter()
        .map(|v| {

            let mut temp_r = r.clone();
            temp_r[0] = v;
            let mut out: Vec<u64> = Vec::new();
            let mut pointer: usize = 0;

            loop {
                if pointer >= p.len() {
                    break;
                }

                if p[pointer][0] == 3 && temp_r[0] != 0 {
                    pointer = p[pointer][1] as usize;
                } else {
                    let (r_r, s) = OPCODES[p[pointer][0] as usize](temp_r.clone(), p[pointer][1]);
                    temp_r = r_r;
                    if let Some(value) = s {
                        out.push(value);
                    }
                    pointer += 1;
                }
            }
          // println!("{:#08b},{:?}",v, out);
            out
        }).collect();

    None // Return the value of 'v' if found
}

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<u64>>) {
    let split: Vec<&str> = input.split("\n\n").into_iter().collect();

    let registers: Vec<u64> = split[0]
        .lines()
        .map(|l| l[12..].parse::<u64>().unwrap())
        .collect();
    let program: Vec<Vec<u64>> = split[1][9..]
        .split(",")
        .into_iter()
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| {
            vec![
                chunk[0].parse::<u64>().unwrap(),
                chunk[1].parse::<u64>().unwrap(),
            ]
        })
        .collect();
    (registers, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
