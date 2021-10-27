mod claim;

use std::collections::HashSet;
use claim::Claim;

fn main() {
    let input = aoc2018::read_file("src/bin/03a/input.txt");
    let result = calculate(input);
    println!("{}", result)
}

fn calculate(input: impl Iterator<Item=String>) -> usize {
    let claims = make_claims(input);
    let mut intersecting_squares = HashSet::new();
    for i in 1..claims.len() {
        for j in 0..i {
            if let Some(intersection) = claims[i].intersection(&claims[j]) {
                intersecting_squares.extend(intersection.squares());
            }
        }
    }
    intersecting_squares.len()
}

fn make_claims(input: impl Iterator<Item=String>) -> Vec<Claim> {
    input.map(|s| Claim::new(&s)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn calculate_test() {
        let result = calculate(INPUT.split('\n').map(String::from));
        assert_eq!(result, 4)
    }
}