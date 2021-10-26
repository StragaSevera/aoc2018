use std::{
    fmt::{Display, Formatter},
    error,
    num::ParseIntError,
    str::FromStr,
};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use ClaimError::{FormatError};

#[derive(PartialEq, Eq, Debug)]
pub struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

// #1 @ 1,3: 4x4
lazy_static! {
    static ref CLAIM_REGEX: Regex = Regex::new(
        r"(?x)^
        \#(?P<id>\d+)
        \s@\s
        (?P<x>\d+),
        (?P<y>\d+):\s
        (?P<width>\d+)
        x
        (?P<height>\d+)$"
    ).unwrap();
}

impl FromStr for Claim {
    type Err = ClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = CLAIM_REGEX.captures(s).ok_or(FormatError)?;
        Ok(Claim::new(caps))
    }
}

impl Claim {
    fn new(caps: Captures) -> Claim {
        Self {
            id: caps["id"].parse().unwrap(),
            x: caps["x"].parse().unwrap(),
            y: caps["y"].parse().unwrap(),
            width: caps["width"].parse().unwrap(),
            height: caps["height"].parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClaimError {
    FormatError
}

impl Display for ClaimError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError => { write!(f, "Wrong line format") }
        }
    }
}

impl error::Error for ClaimError {}

#[cfg(test)]
mod tests {
    use super::*;

    const CORRECT_INPUT: &str = "#1 @ 2,3: 5x4";

    #[test]
    fn from_str_correct_test() {
        let expected = Claim {
            id: 1,
            x: 2,
            y: 3,
            width: 5,
            height: 4,
        };
        let result = CORRECT_INPUT.parse::<Claim>().unwrap();
        assert_eq!(result, expected);
    }

    const INCORRECT_FORMAT_INPUT: &str = "@1 # 2,3: 5x4";

    #[test]
    fn from_str_incorrect_format_test() {
        let expected = FormatError;
        let result = INCORRECT_FORMAT_INPUT.parse::<Claim>().unwrap_err();
        assert_eq!(result, expected);
    }
}