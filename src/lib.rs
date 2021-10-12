use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::iter::Map;

pub fn read_file(filename: &str) -> Map<Lines<BufReader<File>>, fn(Result<String>) -> String> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.map(|line| line.unwrap())
}