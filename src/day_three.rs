use std::str::Lines;

pub fn day_three(input: Lines) -> i32 {
    let lower: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    let upper: String = lower.to_ascii_uppercase();

    let mut matches: Vec<char> = Vec::new();

    for line in input {
        let split = line.len() / 2;
        let compartment_one: String = line.chars().skip(0).take(split).collect();
        let compartment_two: String = line.chars().skip(split).take(line.len()).collect();

        for char in compartment_one.chars() {
            if compartment_two.contains(char) {
                matches.push(char);
                break;
            }
        }
    }

    let mut sum: i32 = 0;

    for char in matches {
        let lower_match = lower.find(char).unwrap_or(0) as i32;
        let upper_match = upper.find(char).unwrap_or(0) as i32;

        if lower_match > 0 {
            sum = sum + (lower_match + 1);
        } else if upper_match > 0 {
            sum = sum + (lower.len() as i32 + 1 + upper_match);
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::utils;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base_case() {
        let input = utils::read_test_file("day_three", "sample.txt");

        assert_eq!(day_three(input.lines()), 157);
    }

    #[test]
    fn test_part_one() {
        let input = utils::read_test_file("day_three", "puzzle.txt");

        assert_eq!(day_three(input.lines()), 7889);
    }
}
