mod claim;

use claim::Claim;

fn main() {
    let input = aoc2018::read_file("src/bin/03b/input.txt");
    let result = calculate(input);
    println!("{}", result)
}

fn calculate(input: impl Iterator<Item=String>) -> u32 {
    let claims = make_claims(input);
    let mut result: Option<&Claim> = None;
    'outer: for i in 0..claims.len() {
        for j in 0..claims.len() {
            if i == j { continue; }
            if claims[i].intersects_with(&claims[j]) { continue 'outer; }
        }
        result = Some(&claims[i]);
        break;
    };
    result.unwrap().id
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
        assert_eq!(result, 3)
    }
}