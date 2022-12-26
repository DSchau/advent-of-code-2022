use std::{collections::HashMap, path::PathBuf};

// thank you https://github.com/NickyMeuleman/scrapyard/blob/main/advent_of_code/2022/src/day_07.rs
pub fn part_one(input: String) -> String {
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }

    sizes
        .into_values()
        .filter(|size| *size <= 100_000)
        .sum::<u32>()
        .to_string()
}

fn part_two(input: String) -> String {
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }

    let disk = 70_000_000;
    let needed = 30_000_000;
    let root = sizes.get(&PathBuf::from("/")).unwrap();
    let available = disk - root;

    sizes
        .into_values()
        .filter(|size| available + size >= needed)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_get_tree_day_seven_sample() {
        let input = utils::read_test_file("day_seven", "sample.txt");

        let result = part_one(input);

        assert_eq!(result, "95437");
    }

    #[test]
    fn test_sample_day_seven_input() {
        let input = utils::read_test_file("day_seven", "puzzle.txt");

        let result = part_one(input);

        assert_eq!(result, "1086293");
    }

    #[test]
    fn test_sample_day_seven_input_part_two() {
        let input = utils::read_test_file("day_seven", "puzzle.txt");

        let result = part_two(input);

        assert_eq!(result, "366028");
    }
}
