use std::{
    fmt::{Display, Formatter},
    error,
    str::FromStr,
};
use std::cmp::{max, min};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use ClaimError::{FormatError};

type Square = (u32, u32);

#[derive(PartialEq, Eq, Debug)]
pub struct Claim {
    id: u32,
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

    pub fn intersection(&self, other: &Claim) -> Option<Claim> {
        if (self.x1 > other.x2) || (other.x1 > self.x2) ||
            (self.y1 > other.y2) || (other.y1 > self.y2) {
            return None;
        }
        Some(Self {
            id: 0,
            x1: max(self.x1, other.x1),
            y1: max(self.y1, other.y1),
            x2: min(self.x2, other.x2),
            y2: min(self.y2, other.y2),
        })
    }

    fn area(&self) -> u32 {
        let width = self.x2 - self.x1 + 1;
        let height = self.y2 - self.y1 + 1;
        width * height
    }

    pub fn squares(&self) -> Vec<Square> {
        let mut result = Vec::with_capacity(self.area() as usize);
        for i in self.x1..=self.x2 {
            for j in self.y1..=self.y2 {
                result.push((i, j));
            }
        }
        result
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

    const INTERSECTION_SOME: [&str; 2] = [
        "#1 @ 1,3: 4x4",
        "#2 @ 3,1: 4x4"
    ];

    #[test]
    fn intersection_some_test() {
        let a = Claim::new(INTERSECTION_SOME[0]);
        let b = Claim::new(INTERSECTION_SOME[1]);
        let result = a.intersection(&b);

        let expected = Some(Claim {
            id: 0,
            x1: 3,
            y1: 3,
            x2: 4,
            y2: 4,
        });
        assert_eq!(result, expected);
    }

    const INTERSECTION_NONE: [&str; 2] = [
        "#1 @ 1,3: 4x4",
        "#3 @ 5,5: 2x2"
    ];

    #[test]
    fn intersection_none_test() {
        let a = Claim::new(INTERSECTION_NONE[0]);
        let b = Claim::new(INTERSECTION_NONE[1]);
        let result = a.intersection(&b);

        assert_eq!(result, None);
    }

    #[test]
    fn area_test() {
        let claim = Claim::new(CORRECT_INPUT);
        let result = claim.area();

        assert_eq!(result, 20)
    }

    const SQUARES_INPUT: &str = "#1 @ 2,3: 2x3";
    lazy_static! {
        static ref SQUARES_RESULT: Vec<Square> = vec!(
            (2, 3), (2, 4), (2, 5), (3, 3), (3, 4), (3, 5)
        );
    }

    #[test]
    fn squares_test() {
        let claim = Claim::new(SQUARES_INPUT);
        let result = claim.squares();

        assert_eq!(result.len(), SQUARES_RESULT.len());
        result.iter().zip(SQUARES_RESULT.iter()).for_each(|(a, b)| assert_eq!(a, b));
    }
}