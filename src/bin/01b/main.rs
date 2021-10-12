use std::collections::HashSet;

fn main() {
    let input = aoc2018::read_file("src/bin/01b/input.txt");
    let result = calibrate(input);
    println!("{}", result)
}

fn calibrate(input: impl Iterator<Item=String>) -> i32 {
    let numbers: Vec<_> = input.map(|line| line.parse::<i32>().unwrap()).collect();

    let mut set = HashSet::new();
    set.insert(0);

    numbers.into_iter().cycle().try_fold((0, set), |(prev_sum, mut set), num| {
        let next_sum = prev_sum + num;

        if set.insert(next_sum) {
            Result::Ok((next_sum, set))
        } else {
            Result::Err((next_sum, set))
        }
    }).unwrap_err().0
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case("+1, -2, +3, +1" => 2)]
    #[test_case("+1, -1" => 0)]
    #[test_case("+3, +3, +4, -2, -4" => 10)]
    #[test_case("-6, +3, +8, +5, -6" => 5)]
    #[test_case("+7, +7, -2, -7, -4" => 14)]
    fn calibrate_tests(input: &str) -> i32 {
        calibrate(input.split(", ").map(String::from))
    }
}