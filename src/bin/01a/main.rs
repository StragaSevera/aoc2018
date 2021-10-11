use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::iter::Map;

fn main() {
    let input = read_file("src/bin/01a/input.txt");
    let result = calibrate(input);
    println!("{}", result)
}

fn read_file(filename: &str) -> Map<Lines<BufReader<File>>, fn(Result<String>) -> String> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.map(|line| line.unwrap())
}

fn calibrate(input: impl Iterator<Item = String>) -> i32 {
    let numbers = input.map(|line| line.parse::<i32>().unwrap());
    numbers.fold(0, |a, b| a + b)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case( "+1, -2, +3, +1" => 3)]
    #[test_case( "+1, +1, +1" => 3)]
    #[test_case( "+1, +1, -2" => 0)]
    #[test_case( "-1, -2, -3" => -6)]
    fn calibrate_tests(input: &str) -> i32 {
        calibrate(input.split(", ").map(String::from))
    }
}