
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::RegexSet;
use std::fs::File;
use std::io::{self, Read};
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
    Sleeps(i32),
    Wakes(i32),
}

impl FromStr for Event
{
    type Err = io::Error;

    fn from_str(input: &str) -> io::Result<Event>
    {
        lazy_static! {
            static ref RE: RegexSet = RegexSet::new(&[
                // r#"^.+:([0-9]{2}) (Guard #([0-9]+) begins|falls a(sleep)|(wake)s up).*$"#
                r#"^.+:([0-9]{2}\]) "#,
                r#"^.+ Guard.*$"#,
                r#"^.+ falls asleep$"#,
                r#"^.+ wakes up$"#,
            ]).unwrap();
        }
        let matches = RE.captures(input);
        if !matches.matched_any() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, input));
        }
println!("matches: {:?}", matches);
        Ok(Event::BeginsShift(0))
    }
}

struct Guard
{
    id: i32,
    sleeps: [i16; 60],
}

fn main() -> io::Result<()>
{
    let input = read_input("sorted_input")?;
    for line in input.lines() {
        let ev: Event = line.parse()?;
        println!("{:?}", ev);
    }
    Ok(())
}
