use std::{
    fmt::{Display, Formatter},
    error,
    num::ParseIntError,
    str::FromStr
};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use ClaimError::{FormatError, ParseError};

#[derive(Debug, Clone)]
pub enum ClaimError {
    FormatError,
    ParseError(ParseIntError),
}

impl Display for ClaimError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError => { write!(f, "Wrong line format") }
            ParseError(err) => { Display::fmt(&err, f) }
        }
    }
}

impl error::Error for ClaimError {}

impl From<ParseIntError> for ClaimError {
    fn from(error: ParseIntError) -> Self {
        ParseError(error)
    }
}

struct Claim {
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
        Ok(Claim::new(caps)?)
    }
}

impl Claim {
    fn new(caps: Captures) -> Result<Claim, ParseIntError> {
        Ok(Self {
            id: caps["id"].parse()?,
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            width: caps["width"].parse()?,
            height: caps["height"].parse()?,
        })
    }
}