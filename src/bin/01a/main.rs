fn main() {
    let input = aoc2018::read_file("src/bin/01a/input.txt");
    let result = calibrate(input);
    println!("{}", result)
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