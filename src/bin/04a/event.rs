use chrono::prelude::*;
use regex::{Captures, Regex};
use lazy_static::lazy_static;

#[derive(Eq, PartialEq, Debug)]
struct Event {
    pub time: NaiveDateTime,
    pub content: EventContent,
}

#[derive(Eq, PartialEq, Debug)]
enum EventContent {
    GuardShift(u32),
    FallAsleep,
    WakeUp,
}

// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up
impl Event {
    pub fn new(s: &str) -> Self {
        lazy_static! {
            static ref EVENT_REGEX: Regex = Regex::new(
r"(?xm)^
\[(?P<time>
  \d{4}-\d{2}-\d{2}
  \s
  \d{2}:\d{2}
)\]
\s
(?:(?:Guard\s\#
  (?P<id>\d+)
\ begins\ shift$)
|
(?P<sleep>falls\ asleep$)
|
(?P<wake>wakes\ up$))"
            ).unwrap();
        }
        let caps = EVENT_REGEX.captures(s).unwrap();
        let time = Self::build_time(&caps["time"]);
        let content = Self::build_content(caps);
        Self { time, content }
    }

    fn build_time(s: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M").unwrap()
    }


    fn build_content(caps: Captures) -> EventContent {
        if let Some(id) = caps.name("id") {
            EventContent::GuardShift(id.as_str().parse().unwrap())
        } else if let Some(_) = caps.name("sleep") {
            EventContent::FallAsleep
        } else if let Some(_) = caps.name("wake") {
            EventContent::WakeUp
        } else {
            panic!("Wrong string format!")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const CORRECT_GUARD_INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift";

    #[test]
    fn from_str_correct_guard_test() {
        let result = Event::new(CORRECT_GUARD_INPUT);
        let expected = Event {
            time: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
            content: EventContent::GuardShift(10),
        };
        assert_eq!(result, expected);
    }

    const CORRECT_SLEEP_INPUT: &str = "[1518-11-01 00:05] falls asleep";

    #[test]
    fn from_str_correct_sleep_test() {
        let result = Event::new(CORRECT_SLEEP_INPUT);
        let expected = Event {
            time: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 5, 0),
            content: EventContent::FallAsleep,
        };
        assert_eq!(result, expected);
    }

    const CORRECT_WAKE_INPUT: &str = "[1518-11-01 00:25] wakes up";

    #[test]
    fn from_str_correct_wake_test() {
        let result = Event::new(CORRECT_WAKE_INPUT);
        let expected = Event {
            time: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 25, 0),
            content: EventContent::WakeUp,
        };
        assert_eq!(result, expected);
    }
}