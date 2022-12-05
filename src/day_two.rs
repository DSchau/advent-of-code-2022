use std::collections::HashMap;

pub fn get_input_as_array(input: String) -> Vec<Vec<String>> {
    let lines = input.lines();
    let mut arr_of_inputs = Vec::new();

    for line in lines {
        let split_pair_collection = line
            .split(" ")
            .filter(|part| part != &"" && part != &" ")
            .map(|part| part.to_string())
            .collect();

        arr_of_inputs.push(split_pair_collection);
    }

    arr_of_inputs
}

pub fn augment_strategy_array(input: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut new_strategy: Vec<Vec<String>> = Vec::new();

    let strategy_lookup = HashMap::from([
        ("X", "LOSE"), // rock
        ("Y", "DRAW"), // paper
        ("Z", "WIN"),  // scissors
    ]);

    let opponent_lookup = HashMap::from([
        ("A", "X"), // rock
        ("B", "Y"), // paper
        ("C", "Z"), // scissors
    ]);

    for part in input {
        let mut new_pair: Vec<String> = Vec::new();

        let left_value = &part[0];
        let left = opponent_lookup
            .get(&left_value as &str)
            .unwrap_or(&"X")
            .to_string();
        let right = &part[1];

        let strategy = strategy_lookup.get(&right as &str).unwrap_or(&"DRAW");

        new_pair.push(left_value.to_string());

        let mut new_right_value: &str = &left.to_string();

        if strategy == &"LOSE" {
            if left == "X" {
                new_right_value = "Z";
            } else if left == "Y" {
                new_right_value = "X";
            } else if left == "Z" {
                new_right_value = "Y";
            }
        } else if strategy == &"WIN" {
            if left == "X" {
                new_right_value = "Y";
            } else if left == "Y" {
                new_right_value = "Z";
            } else if left == "Z" {
                new_right_value = "X";
            }
        }

        new_pair.push(new_right_value.to_string());

        new_strategy.push(new_pair);
    }

    new_strategy
}

pub fn get_score(input: Vec<Vec<String>>) -> i32 {
    let opponent_lookup = HashMap::from([
        ("A", "X"), // rock
        ("B", "Y"), // paper
        ("C", "Z"), // scissors
    ]);

    let score_lookup = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let outcome_score_lookup = HashMap::from([("WIN", 6), ("DRAW", 3), ("LOSE", 0)]);

    let mut score: i32 = 0;

    for part in input {
        let left_value = &part[0];
        let left = &opponent_lookup
            .get(left_value as &str)
            .as_deref()
            .unwrap_or(&"X")
            .to_string();
        let right = &part[1];

        let mut outcome = "LOSE";

        if right == left {
            outcome = "DRAW";
        }

        if right == "X" && left == &"Z" {
            outcome = "WIN";
        } else if right == "Y" && left == &"X" {
            outcome = "WIN";
        } else if right == "Z" && left == &"Y" {
            outcome = "WIN"
        }

        // this whole next section doesn't make as much sense as I would like
        let outcome_score = outcome_score_lookup.get(outcome);
        let play_score = score_lookup.get(right as &str);

        score = score + outcome_score.unwrap_or(&0) + play_score.unwrap_or(&0);
    }

    score
}

#[cfg(test)]
mod tests {
    use crate::utils;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input() {
        let input = utils::read_test_file("day_two", "puzzle.txt");

        let arr = get_input_as_array(input);

        assert_eq!(arr.len(), 2500);
        assert_eq!(arr[0], vec!["A", "Y"]);
    }

    #[test]
    fn test_get_score_with_basic_input() {
        let input = utils::read_test_file("day_two", "puzzle.txt");

        let arr = get_input_as_array(input);

        let score = get_score(arr);

        assert_eq!(score, 15422);
    }

    #[test]
    fn test_strategy_arr() {
        let input = utils::read_test_file("day_two", "sample.txt");

        let arr = get_input_as_array(input);
        let strategy_arr = augment_strategy_array(arr);

        assert_eq!(
            strategy_arr[0],
            vec!["A", "X"],
            "expect strategy to be draw, A = X"
        ); // Draw, rock = rock
        assert_eq!(
            strategy_arr[1],
            vec!["B", "X"],
            "expect strategy to be loss, B > X"
        ); // Lose, paper > rock
        assert_eq!(
            strategy_arr[2],
            vec!["C", "X"],
            "expect strategy to be win, C < X"
        ); // Win, rock > scissors
    }

    #[test]
    fn test_strategy_arr_complex() {
        let input = utils::read_test_file("day_two", "puzzle.txt");

        let arr = get_input_as_array(input);
        let strategy_arr = augment_strategy_array(arr);

        assert_eq!(
            strategy_arr[0],
            vec!["A", "X"],
            "expect strategy to be draw, A = X"
        ); // Draw, rock = rock
        assert_eq!(
            strategy_arr[1],
            vec!["B", "Y"],
            "expect strategy to be draw, B = Y"
        ); // Draw, paper = paper
        assert_eq!(
            strategy_arr[2],
            vec!["B", "Z"],
            "expect strategy to be win, B > X"
        ); // Win, scissors > paper
    }

    #[test]
    fn test_get_score_with_part_two_strategy_simple() {
        let input = utils::read_test_file("day_two", "sample.txt");

        let arr = get_input_as_array(input);
        let strategy_arr = augment_strategy_array(arr);

        let score = get_score(strategy_arr);

        assert_eq!(score, 12);
    }

    #[test]
    fn test_get_score_with_part_two_strategy() {
        let input = utils::read_test_file("day_two", "puzzle.txt");

        let arr = get_input_as_array(input);
        let strategy_arr = augment_strategy_array(arr);

        let score = get_score(strategy_arr);

        assert_eq!(score, 15442);
    }
}
