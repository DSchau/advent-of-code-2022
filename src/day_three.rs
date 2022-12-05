use std::str::Lines;

pub fn get_score(ch: char) -> i32 {
  let lower: String = "abcdefghijklmnopqrstuvwxyz".to_string();
  let upper: String = lower.to_ascii_uppercase();

  let lower_match = lower.find(ch).unwrap_or(0) as i32;
  let upper_match = upper.find(ch).unwrap_or(0) as i32;

  let mut score = 0;

  if lower_match > 0 {
    score = lower_match + 1;
  } else if upper_match > 0 {
    score = lower.len() as i32 + 1 + upper_match;
  }

  score
}

pub fn get_rucksack_sum(input: Lines) -> i32 {
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

    matches.iter()
      .fold(0, |acc, ch| acc + get_score(*ch))
}

pub fn get_rucksam_sum_with_triples(input: Lines) -> i32 {
  let mut clone = input.clone();
  let mut matches: Vec<char> = Vec::new();

  while let (Some(line1), Some(line2), Some(line3)) = (clone.next(), clone.next(), clone.next()) {
    for ch in line1.chars() {
      if line2.contains(ch) && line3.contains(ch) {
        matches.push(ch);
        break;
      }
    }
  }

  matches.iter()
    .fold(0, |acc, ch| acc + get_score(*ch))
}

#[cfg(test)]
mod tests {
    use crate::utils;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base_case() {
        let input = utils::read_test_file("day_three", "sample.txt");

        assert_eq!(get_rucksack_sum(input.lines()), 157);
    }

    #[test]
    fn test_part_one() {
        let input = utils::read_test_file("day_three", "puzzle.txt");

        assert_eq!(get_rucksack_sum(input.lines()), 7889);
    }

    #[test]
    fn test_part_two_simple() {
        let input = utils::read_test_file("day_three", "sample.txt");

        assert_eq!(get_rucksam_sum_with_triples(input.lines()), 70);
    }

    #[test]
    fn test_part_two() {
        let input = utils::read_test_file("day_three", "puzzle.txt");

        assert_eq!(get_rucksam_sum_with_triples(input.lines()), 2825);
    }
}
