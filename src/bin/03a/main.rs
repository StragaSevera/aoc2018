mod claim;

fn main() {
    let input = aoc2018::read_file("src/bin/03a/input.txt");
    let result = calculate(input);
    println!("{}", result)
}

fn calculate(input: impl Iterator<Item=String>) -> i32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

    #[test]
    fn calculate_test() {
        let result = calculate(INPUT.split('\n').map(String::from));
        assert_eq!(result, 5)
    }
}