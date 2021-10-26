use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::iter::Map;

type FileIterator = Map<Lines<BufReader<File>>, fn(Result<String>) -> String>;

pub fn read_file(filename: &str) -> FileIterator {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.map(|line| line.unwrap())
}