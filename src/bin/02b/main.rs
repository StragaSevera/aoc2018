use std::iter::FromIterator;

fn main() {
    let input = aoc2018::read_file("src/bin/02b/input.txt");
    let result = find_correct_id(input);
    println!("{}", result)
}

fn find_correct_id(input: impl Iterator<Item=String>) -> String {
    let mut strings: Vec<Vec<char>> = Vec::new();
    let input = input.map(|s| s.chars().collect::<Vec<_>>());

    let mut diff_idx: Option<usize> = None;
    let mut result: Option<Vec<char>> = None;

    'outer: for input_line in input {
        if !strings.is_empty() {
            for string in &strings {
                for (i, (a, b)) in input_line.iter().zip(string).enumerate() {
                    if *a != *b {
                        match diff_idx {
                            None => diff_idx = Some(i),
                            Some(_) => {
                                diff_idx = None;
                                break;
                            }
                        }
                    }
                }
                if diff_idx.is_some() {
                    result = Some(input_line);
                    break 'outer;
                }
            }
        }
        strings.push(input_line)
    }

    let mut result = result.unwrap();
    result.remove(diff_idx.unwrap());
    String::from_iter(result)
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
    fn find_correct_id_test() {
        let result = find_correct_id(INPUT.split('\n').map(String::from));
        assert_eq!(result, "fgij")
    }
}