pub enum Strategy {
    All,
    Any,
}

fn normalize_str_pairs(pair: Vec<&str>) -> Vec<i32> {
    pair.iter().map(|val| val.parse::<i32>().unwrap()).collect()
}

fn overlaps(left: &Vec<i32>, right: &Vec<i32>, strategy: &Strategy) -> bool {
    let min_left = left[0];
    let max_left = left[1];

    let min_right = right[0];
    let max_right = right[1];

    match strategy {
        Strategy::All => {
            min_left >= min_right && max_left <= max_right
                || min_right >= min_left && max_right <= max_left
        }
        Strategy::Any => {
            let mut found = false;
            for num in min_left..max_left + 1 {
                if num >= min_right && num <= max_right {
                    found = true;
                    break;
                }
            }

            for num in min_right..max_right + 1 {
                if num >= min_left && num <= max_left {
                    found = true;
                    break;
                }
            }

            found
        }
    }
}

pub fn get_overlapping_ranges(input: String, strategy: Strategy) -> i32 {
    let lines = input.lines();

    let mut matches: Vec<String> = Vec::new();

    for line in lines {
        let pair = line.split(",").collect::<Vec<&str>>();

        let left = normalize_str_pairs(pair[0].split("-").collect::<Vec<&str>>());
        let right = normalize_str_pairs(pair[1].split("-").collect::<Vec<&str>>());

        if overlaps(&left, &right, &strategy) {
            matches.push(line.to_string());
        }
    }

    matches.len() as i32
}

#[cfg(test)]
mod tests {
    use crate::utils;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base_case() {
        let input = utils::read_test_file("day_four", "sample.txt");

        assert_eq!(get_overlapping_ranges(input, Strategy::All), 2);
    }

    #[test]
    fn test_puzzle_input_day_four() {
        let input = utils::read_test_file("day_four", "puzzle.txt");

        assert_eq!(get_overlapping_ranges(input, Strategy::All), 518);
    }

    #[test]
    fn test_base_case_part_two() {
        let input = utils::read_test_file("day_four", "sample.txt");

        assert_eq!(get_overlapping_ranges(input, Strategy::Any), 4);
    }

    #[test]
    fn test_puzzle_input_day_four_part_two() {
        let input = utils::read_test_file("day_four", "puzzle.txt");

        assert_eq!(get_overlapping_ranges(input, Strategy::Any), 909);
    }
}
