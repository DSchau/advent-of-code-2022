pub fn day_seven(input: String) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_sample_day_seven() {
        let input = utils::read_test_file("day_seven", "sample.txt");

        assert_eq!(day_seven(input), -1); // 95437
    }
}
