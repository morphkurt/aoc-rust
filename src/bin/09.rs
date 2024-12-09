advent_of_code::solution!(9);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Block {
    start: i32,
    file_id: i32,
    file_len: i32,
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut file_system = parse(input);
    let mut l: usize = 0;
    let mut r: usize = file_system.len() - 1;
    loop {
        while file_system[l] != -1 && l < file_system.len() {
            l += 1;
        }
        while file_system[r] == -1 && r > 0 {
            r -= 1;
        }
        if l > r {
            break;
        }
        file_system.swap(l, r);
    }
    let mut sum = 0;
    for i in 0..file_system.len() {
        if file_system[i] != -1 {
            sum += i * file_system[i] as usize;
        }
    }
    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut free_space, used_space) = parse_part2(input);
    let mut final_space: Vec<Block> = Vec::new();

    for i in 0..used_space.len() {
        let index = used_space.len() - i - 1;
        let mut found = false;
        for j in 0..free_space.len() {
            if used_space[index].start < free_space[j].start {
                break;
            }
            let free = &free_space[j];
            if free.file_len > used_space[index].file_len {
                final_space.push(Block {
                    start: free.start,
                    file_id: used_space[index].file_id,
                    file_len: used_space[index].file_len,
                });
                free_space[j].start += used_space[index].file_len;
                free_space[j].file_len -= used_space[index].file_len;

                found = true;
                break;
            } else if free.file_len == used_space[index].file_len {
                final_space.push(Block {
                    start: free.start,
                    file_id: used_space[index].file_id,
                    file_len: used_space[index].file_len,
                });
                free_space.remove(j);
                found = true;
                break;
            }
        }
        if !found {
            final_space.push(used_space[index].clone());
        }
    }

    let sum = final_space
        .iter()
        .map(|block| {
            (block.start..block.start + block.file_len)
                .map(|i| i as u64 * block.file_id as u64)
                .sum::<u64>()
        })
        .sum();
    return Some(sum);
}

fn parse(input: &str) -> Vec<i32> {
    let mut result = Vec::new();
    let mut file_id = 0;

    for chunk in input.chars().collect::<Vec<_>>().chunks(2) {
        if let Some(&file_len_char) = chunk.get(0) {
            let file_len = file_len_char.to_digit(10).unwrap() as usize;
            result.extend(vec![file_id; file_len]);
        }
        if let Some(&free_space_char) = chunk.get(1) {
            let free_space = free_space_char.to_digit(10).unwrap() as usize;
            result.extend(vec![-1; free_space]);
        }
        file_id += 1;
    }
    result
}

fn parse_part2(input: &str) -> (Vec<Block>, Vec<Block>) {
    let mut free_space = Vec::new();
    let mut used_space = Vec::new();

    let mut file_id = 0;

    let mut index: usize = 0;

    for chunk in input.chars().collect::<Vec<_>>().chunks(2) {
        if let Some(&file_len_char) = chunk.get(0) {
            let len = file_len_char.to_digit(10).unwrap() as i32;
            used_space.push(Block {
                start: index as i32,
                file_id,
                file_len: len,
            });
            index += len as usize;
        }
        if let Some(&free_space_char) = chunk.get(1) {
            let len = free_space_char.to_digit(10).unwrap() as i32;
            free_space.push(Block {
                start: index as i32,
                file_id: -1,
                file_len: len,
            });
            index += len as usize;
        }
        file_id += 1;
    }
    return (free_space, used_space);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
