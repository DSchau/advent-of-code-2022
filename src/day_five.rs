pub fn get_arrangement(input: String) -> Vec<String> {
    let output = Vec::new();

    output
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_base_case() {
        let input = utils::read_test_file("day_five", "sample.txt");

        let expected: Vec<String> = Vec::new();

        assert_eq!(get_arrangement(input), expected);
    }
}
