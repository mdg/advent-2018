
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::ops::Range;
use std::str::FromStr;


fn read_input(input_filename: &str) -> io::Result<String>
{
    let mut f = File::open(input_filename)?;
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Event
{
    BeginsShift(i32),
    Sleeps(i16),
    Wakes(i16),
}

impl FromStr for Event
{
    type Err = io::Error;

    fn from_str(input: &str) -> io::Result<Event>
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r#"^.+:([0-9]{2})\] (Guard #([0-9]+) begins|falls a(sleep)|(wake)s up).*$"#
            ).unwrap();
        }
        let opt_matches = RE.captures(input);
        if opt_matches.is_none() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, input));
        }
        let matches = opt_matches.as_ref().unwrap();
        let minute_match = matches.get(1).unwrap();
        let guard_match = matches.get(3);
        let sleep_match = matches.get(4);

        let minute_str = minute_match.as_str();
        let minute = i16::from_str_radix(minute_str, 10).unwrap();

        let ev = if guard_match.is_some() {
            let guard_str = guard_match.unwrap().as_str();
            let id = i32::from_str_radix(guard_str, 10).unwrap();
            Event::BeginsShift(id)
        } else if sleep_match.is_some() {
            Event::Sleeps(minute)
        } else {
            Event::Wakes(minute)
        };
        Ok(ev)
    }
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
struct Sleep
{
    id: i32,
    sleep: i16,
    wake: i16,
}

impl Sleep
{
    pub fn range(&self) -> Range<usize>
    {
        let first = self.sleep as usize;
        let second = self.wake as usize;
        first..second
    }

    pub fn duration(&self) -> i32
    {
        (self.wake - self.sleep) as i32
    }
}

#[derive(Debug)]
struct Guard
{
    id: i32,
    total: i32,
    sleeps: Vec<i16>,
}

impl Guard
{
    pub fn new(s: Sleep) -> Guard
    {
        Guard {
            id: s.id,
            total: 0,
            sleeps: vec![0; 60],
        }
    }

    pub fn add_sleep(&mut self, s: Sleep)
    {
        for i in s.range() {
            self.sleeps[i] += 1;
        }
        self.total += s.duration();
    }

    pub fn sleepiest_minute(&self) -> i16
    {
        let sleepiest = self.sleeps
            .iter()
            .enumerate()
            .fold((0, -1), |max, minute| {
                if *minute.1 > max.1 {
                    (minute.0, *minute.1)
                } else {
                    max
                }
            });
        sleepiest.0 as i16
    }
}

fn main() -> io::Result<()>
{
    let input = read_input("sorted_input")?;
    let events_result: io::Result<Vec<Event>> = input
        .lines()
        .map(|line| {
            line.parse()
        })
        .collect();
    let events = events_result?;
    let mut sleeps = Vec::with_capacity(events.len());
    let mut guard_id = 0;
    let mut sleep_time = 0;
    for ev in events {
        match ev {
            Event::BeginsShift(id) => {
                guard_id = id;
            }
            Event::Sleeps(minute) => {
                sleep_time = minute;
            }
            Event::Wakes(minute) => {
                sleeps.push(Sleep {
                    id: guard_id,
                    sleep: sleep_time,
                    wake: minute,
                });
            }
        }
    }

    let mut guards = HashMap::new();
    for s in sleeps {
        let mut g = guards.entry(s.id).or_insert(Guard::new(s));
        g.add_sleep(s);
    }

    let sleepiest = guards.values().fold((0, 0, -1), |acc, g| {
        if g.total > acc.2 {
            println!("guard {} is sleepier: {:?} > {:?}", g.id, g.total, acc);
            (g.id, g.sleepiest_minute().into(), g.total)
        } else {
            acc
        }
    });

    println!("sleepiest: {:?}", sleepiest);
    println!("answer: {}", sleepiest.0 * sleepiest.1);
    Ok(())
}
