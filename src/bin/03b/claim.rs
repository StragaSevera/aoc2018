use std::{
    fmt::{Display, Formatter},
    error,
    str::FromStr,
};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use ClaimError::{FormatError};

#[derive(PartialEq, Eq, Debug)]
pub struct Claim {
    pub(crate) id: u32,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
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

impl Claim {
    pub fn new(s: &str) -> Self {
        s.parse().unwrap()
    }

    fn from_caps(caps: Captures) -> Claim {
        let id = caps["id"].parse().unwrap();
        let x1 = caps["x"].parse().unwrap();
        let y1 = caps["y"].parse().unwrap();
        let width: u32 = caps["width"].parse().unwrap();
        let height: u32 = caps["height"].parse().unwrap();
        Self {
            id,
            x1,
            y1,
            x2: x1 + width - 1,
            y2: y1 + height - 1,
        }
    }

    pub fn intersects_with(&self, other: &Claim) -> bool {
        !((self.x1 > other.x2) || (other.x1 > self.x2) ||
            (self.y1 > other.y2) || (other.y1 > self.y2))
    }
}

impl FromStr for Claim {
    type Err = ClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = CLAIM_REGEX.captures(s).ok_or(FormatError)?;
        Ok(Claim::from_caps(caps))
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
    use test_case::test_case;
    use pretty_assertions::assert_eq;

    const CORRECT_INPUT: &str = "#1 @ 2,3: 5x4";

    #[test]
    fn from_str_correct_test() {
        let result = CORRECT_INPUT.parse::<Claim>().unwrap();
        let expected = Claim {
            id: 1,
            x1: 2,
            y1: 3,
            x2: 6,
            y2: 6,
        };
        assert_eq!(result, expected);
    }

    const INCORRECT_FORMAT_INPUT: &str = "@1 # 2,3: 5x4";

    #[test]
    fn from_str_incorrect_format_test() {
        let result = INCORRECT_FORMAT_INPUT.parse::<Claim>().unwrap_err();
        assert_eq!(result, FormatError);
    }

    const INTERSECTION_SOME: (&str, &str) = (
        "#1 @ 1,3: 4x4",
        "#2 @ 3,1: 4x4"
    );
    const INTERSECTION_NONE: (&str, &str) = (
        "#1 @ 1,3: 4x4",
        "#3 @ 5,5: 2x2"
    );

    #[test_case(INTERSECTION_SOME, true)]
    #[test_case(INTERSECTION_NONE, false)]
    fn intersects_with_test((a, b): (&str, &str), result: bool) {
        let a = Claim::new(a);
        let b = Claim::new(b);
        pretty_assertions::assert_eq!(a.intersects_with(&b), result);
    }
}