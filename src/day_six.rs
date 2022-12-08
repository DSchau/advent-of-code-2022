pub fn is_unique(input: String) -> bool {
    let collection: Vec<char> = input.chars().collect();

    let mut clone: Vec<char> = collection.clone();

    clone.sort();
    clone.dedup();

    println!("{:?}{:?}", collection, clone);

    clone.len() == collection.len()
}

pub fn day_six(input: String, range_size: usize) -> i32 {
    let chars = input.chars();

    for (index, _char) in chars.enumerate() {
        let sub_str: String = input.chars().skip(index).take(range_size).collect();

        if sub_str.len() == range_size && is_unique(sub_str) {
            return (index + range_size) as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;
    struct Input<'a> {
        input: &'a str, // no idea, very confused
        expected: i32,
    }

    #[test]
    fn test_samples_day_six() {
        let samples = vec![
            Input {
                input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                expected: 7,
            },
            Input {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
                expected: 5,
            },
            Input {
                input: "nppdvjthqldpwncqszvftbrmjlhg",
                expected: 6,
            },
            Input {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                expected: 10,
            },
            Input {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                expected: 11,
            },
        ];

        for sample in samples {
            assert_eq!(day_six(sample.input.to_string(), 4), sample.expected);
        }
    }

    #[test]
    fn test_puzzle_day_six() {
        let input = utils::read_test_file("day_six", "puzzle.txt");

        assert_eq!(day_six(input, 4), 1647);
    }

    #[test]
    fn test_samples_part_two_day_six() {
        let samples = vec![
            Input {
                input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                expected: 19,
            },
            Input {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
                expected: 23,
            },
            Input {
                input: "nppdvjthqldpwncqszvftbrmjlhg",
                expected: 23,
            },
            Input {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                expected: 29,
            },
            Input {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                expected: 26,
            },
        ];

        for sample in samples {
            assert_eq!(day_six(sample.input.to_string(), 14), sample.expected);
        }
    }

    #[test]
    fn test_puzzle_part_two_day_six() {
        let input = utils::read_test_file("day_six", "puzzle.txt");

        assert_eq!(day_six(input, 14), 2447);
    }
}
