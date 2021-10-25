use std::collections::HashMap;

fn main() {
    let input = aoc2018::read_file("src/bin/02a/input.txt");
    let result = find_checksum(input);
    println!("{}", result)
}

fn find_checksum(input: impl Iterator<Item=String>) -> i32 {
    let (times_2, times_3) = input.fold((0, 0), |acc, string| {
        let mut increment_2: i32 = 0;
        let mut increment_3: i32 = 0;

        let result = string.chars().fold(HashMap::new(), |mut acc, chr| {
            acc.entry(chr).and_modify(|i| *i += 1).or_insert(1);
            acc
        });

        for (_, i) in result {
            if i == 2 { increment_2 = 1 };
            if i == 3 { increment_3 = 1 };
            if increment_2 == 1 && increment_3 == 1 { break; };
        }

        (acc.0 + increment_2, acc.1 + increment_3)
    });
    times_2 * times_3
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    #[test]
    fn find_checksum_test() {
        let result = find_checksum(INPUT.split("\n").map(String::from));
        assert_eq!(result, 12)
    }
}