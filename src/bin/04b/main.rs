use std::collections::HashMap;
use std::ops::Range;
use arrayvec::ArrayVec;
use chrono::{Duration, NaiveDateTime, Timelike};
use event::Event;
use event::State;
// use lazy_static::lazy_static;

mod event;

fn main() {
    let input = aoc2018::read_file("src/bin/04b/input.txt");
    let result = calculate(input);
    println!("{}", result)
}

fn calculate(input: impl Iterator<Item=String>) -> u32 {
    let mut events: Vec<Event> = input.map(|s| Event::new(&s)).collect();
    events.sort_unstable_by_key(|e| e.datetime);
    let guards: HashMap<u32, Vec<Range<u32>>> = build_guards(events);

    let guards_by_minutes = guards.iter().map(|(id, timeline)| {
        let result: ArrayVec<u32, 60> = (0u32..60).map(|i| {
            timeline.iter().filter(|e| e.contains(&i)).count() as u32
        }).collect();
        (*id, result)
    });

    let guards_max_minutes = guards_by_minutes.map(|(id, minutes)| {
        let (minute, amount) = minutes.iter().enumerate().max_by_key(|(_, v)| *v).unwrap();
        (id, minute, *amount)
    });

    let (sleepy_guard_id, sleepy_minute, _) = guards_max_minutes.max_by_key(|(_, _, amount)| *amount).unwrap();

    sleepy_guard_id * sleepy_minute as u32
}

fn build_guards(events: Vec<Event>) -> HashMap<u32, Vec<Range<u32>>> {
    let mut guards: HashMap<u32, Vec<Range<u32>>> = HashMap::new();
    let mut current_guard: Option<u32> = None;
    let mut asleep_event: Option<Event> = None;

    for event in events {
        if let State::GuardShift(guard) = event.state {
            current_guard = Some(guard);
            guards.entry(guard).or_insert_with(Vec::new);
            asleep_event = None;
        } else if let Some(guard) = current_guard {
            match &asleep_event {
                None => {
                    if event.state == State::FallAsleep { asleep_event = Some(event); }
                }
                Some(unwrapped_event) => {
                    if event.state == State::WakeUp {
                        let (sleep_start, sleep_end) = build_sleep_bounds(event, &unwrapped_event);
                        guards.get_mut(&guard).unwrap().push(
                            sleep_start.minute()..sleep_end.minute());
                        asleep_event = None
                    }
                }
            }
        }
    }

    guards
}

fn build_sleep_bounds(event: Event, asleep_event: &&Event) -> (NaiveDateTime, NaiveDateTime) {
    let midnight = hour_for_date(&asleep_event.datetime, 0);
    let one_am = hour_for_date(&asleep_event.datetime, 1);
    let sleep_start = asleep_event.datetime.clamp(midnight, one_am);
    let sleep_end = event.datetime.clamp(midnight, one_am);
    (sleep_start, sleep_end)
}

fn hour_for_date(datetime: &NaiveDateTime, hour: u32) -> NaiveDateTime {
    if datetime.hour() < 12 {
        (datetime.date()).and_hms(hour, 0, 0)
    } else {
        (datetime.date() + Duration::days(1)).and_hms(hour, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn calculate_test() {
        let result = calculate(INPUT.split('\n').map(String::from));
        assert_eq!(result, 4455)
    }
}