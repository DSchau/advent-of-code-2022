use std::{fs, path::Path};

pub fn read_test_file(day: &str, file: &str) -> String {
    let file_path = Path::new("./src/inputs").join(day).join(file);
    println!("{:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Could not find file at {file_path}");

    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_case() {
        let file_path = read_test_file("day_one", "sample.txt");

        assert_eq!(file_path.lines().count(), 14);
        assert_eq!(file_path.contains("10000"), true);
    }
}
