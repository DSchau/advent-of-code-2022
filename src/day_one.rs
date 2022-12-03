use regex::Regex;

pub fn sanitize_input(input: &str) -> Vec<Vec<i32>> {
    let binding = Regex::new(r"\n").unwrap();
    let mut collection: Vec<Vec<i32>> = Vec::new();
    let mut split = binding.split(input.trim());

    let mut sub_collection: Vec<i32> = Vec::new();

    // .. there has to be an easier way to do this right?
    loop {
        match split.next() {
            Some(part) => {
                if part.chars().count() == 0 {
                    collection.push(sub_collection);
                    sub_collection = Vec::new();
                } else {
                    sub_collection.push(part.parse::<i32>().unwrap());
                }
            }
            None => {
                collection.push(sub_collection);
                break;
            }
        }
    }

    collection
}

pub fn get_largest_character(input: &str) -> i32 {
    let collection = sanitize_input(input);

    let mut largest: i32 = 0;

    for sub_collection in collection {
        let value = sub_collection.iter().fold(0, |acc, part| acc + part);
        if value > largest {
            largest = value
        }
    }
    largest
}

pub fn get_top_characters(input: &str) -> i32 {
    let top = 3; // hard_coding, blah
    let collection = sanitize_input(input);
    let mut sorted: Vec<i32> = Vec::new();

    let mut value: i32 = 0;

    for sub_collection in collection {
        let value = sub_collection.iter().fold(0, |acc, part| acc + part);
        sorted.push(value);
    }

    sorted.sort();
    sorted.reverse();

    for index in 0..top {
        let part = &sorted[index];
        value = value + part
    }

    value
}

#[cfg(test)]
mod tests {
    use crate::utils;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sanitize_input() {
        let input: &str = "123\n456\n789";

        let res = sanitize_input(input);

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], vec![123, 456, 789]);
    }

    #[test]
    fn test_sanitize_real_input() {
        let input = utils::read_test_file("day_one", "sample.txt");

        let res = sanitize_input(&input);

        assert_eq!(res.len(), 5);
        assert_eq!(res[0], vec![1000, 2000, 3000]);
        assert_eq!(res[1], vec![4000]);
        assert_eq!(res[2], vec![5000, 6000]);
        assert_eq!(res[3], vec![7000, 8000, 9000]);
        assert_eq!(res[4], vec![10000]);
    }

    #[test]
    fn gets_largest_character_simple() {
        let input = utils::read_test_file("day_one", "sample.txt");

        assert_eq!(get_largest_character(&input), 24000);
    }

    #[test]
    fn gets_largest_character_puzzle_input() {
        let input = utils::read_test_file("day_one", "puzzle.txt");
        assert_eq!(get_largest_character(&input), 70720);
    }

    #[test]
    fn get_char_sequence_simple() {
        let input = utils::read_test_file("day_one", "sample.txt");

        assert_eq!(get_top_characters(&input), 45000);
    }

    #[test]
    fn get_char_sequence_challenging() {
        let input = utils::read_test_file("day_one", "puzzle.txt");

        assert_eq!(get_top_characters(&input), 207148);
    }
}
