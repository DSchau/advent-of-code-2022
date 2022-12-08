use regex::Regex;

pub enum Strategy {
    PartOne,
    PartTwo,
}

#[derive(Debug)]
pub struct Instruction {
    count: i32,
    from: i32,
    to: i32,
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.from == other.from && self.to == other.to
    }
}

#[derive(Debug)]
pub struct Input {
    instructions: Vec<Instruction>,
    stack: Vec<Vec<String>>,
}

pub fn sanitize_stack(input: String) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();

    let lines = input.lines().into_iter();

    let last = lines.last().unwrap().to_string();
    let parts: Vec<&str> = Regex::new(r"\s+").unwrap().split(&last.trim()).collect();

    for _part in parts {
        let stack: Vec<String> = Vec::new();
        output.push(stack);
    }

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        let mut position: i32 = 0;
        let total_size = chars.len() + 1;
        let chunk_size: i32 = ((total_size / output.len()) as f32).ceil() as i32;

        let mut cursor = 0;

        for char in chars {
            if char.is_alphabetic() {
                output[cursor - 1].push(char.to_string());
            }

            // modulus, increment position every new chunk
            if position.rem_euclid(chunk_size) == 0 {
                cursor += 1;
            }

            position += 1;
        }
    }
    output
}

pub fn sanitize_instructions(input: String) -> Vec<Instruction> {
    let mut output: Vec<Instruction> = Vec::new();
    let lines = input.lines();

    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();

        output.push(Instruction {
            count: split[1].parse::<i32>().unwrap(),
            from: split[3].parse::<i32>().unwrap(),
            to: split[5].parse::<i32>().unwrap(),
        })
    }

    output
}

fn get_sanitized_input(input: String) -> Input {
    let clone = input.clone();
    let split = clone.split("\n\n").collect::<Vec<&str>>();

    Input {
        instructions: sanitize_instructions(split[1].to_string()),
        stack: sanitize_stack(split[0].to_string()),
    }
}

pub fn day_five(input: Input, strategy: Strategy) -> Vec<Vec<String>> {
    let mut output = input.stack.clone();

    for instruction in input.instructions {
        let from = (instruction.from - 1) as usize;
        let to = (instruction.to - 1) as usize;
        let count = (instruction.count) as usize;

        match strategy {
            Strategy::PartOne => {
                for _count in 0..instruction.count {
                    let value = output[from].drain(0..1).collect::<Vec<String>>();

                    output[to].splice(0..0, value);
                }
            }
            Strategy::PartTwo => {
                let value = output[from].drain(0..count).collect::<Vec<String>>();

                // output[to].push(value[0].to_string());
                output[to].splice(0..0, value);
            }
        }
    }

    output
}

pub fn get_top(input: Vec<Vec<String>>) -> String {
    let mut output: Vec<String> = Vec::new();

    for stack in input {
        output.push(stack[0].to_string());
    }

    output.join("")
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_sanitize_instructions_day_five() {
        let input = utils::read_test_file("day_five", "sample.txt");

        let split = input.split("\n\n").collect::<Vec<&str>>();

        let output = sanitize_instructions(split[1].to_string());

        assert_eq!(
            output[0],
            Instruction {
                count: 1,
                from: 2,
                to: 1
            }
        );

        assert_eq!(
            output[1],
            Instruction {
                count: 3,
                from: 1,
                to: 3
            }
        );

        assert_eq!(
            output[3],
            Instruction {
                count: 1,
                from: 1,
                to: 2
            }
        );
    }

    #[test]
    fn test_sanitize_stack_day_five() {
        let input = utils::read_test_file("day_five", "sample.txt");

        let split = input.split("\n\n").collect::<Vec<&str>>();

        let output = sanitize_stack(split[0].to_string());

        assert_eq!(output[0], vec!["N", "Z"]);
        assert_eq!(output[1], vec!["D", "C", "M"]);
        assert_eq!(output[2], vec!["P"]);
    }

    #[test]
    fn test_base_case_day_five() {
        let input_str = utils::read_test_file("day_five", "sample.txt");

        let output = day_five(get_sanitized_input(input_str), Strategy::PartOne);

        let top = get_top(output);

        assert_eq!(top, "CMZ");
    }

    #[test]
    fn test_puzzle_case_day_five() {
        let input_str = utils::read_test_file("day_five", "puzzle.txt");

        let output = day_five(get_sanitized_input(input_str), Strategy::PartOne);

        let top = get_top(output);

        assert_eq!(top, "FZCMJCRHZ");
    }

    #[test]
    fn test_base_case_day_five_part_two() {
        let input_str = utils::read_test_file("day_five", "sample.txt");

        let output = day_five(get_sanitized_input(input_str), Strategy::PartTwo);

        let top = get_top(output);

        assert_eq!(top, "MCD");
    }

    #[test]
    fn test_puzzle_case_day_five_part_two() {
        let input_str = utils::read_test_file("day_five", "puzzle.txt");

        let output = day_five(get_sanitized_input(input_str), Strategy::PartTwo);

        let top = get_top(output);

        assert_eq!(top, "JSDHQMZGF");
    }
}
