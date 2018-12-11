
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::usize;


fn read_input(input_filename: &str) -> io::Result<String>
{
    let mut f = File::open(input_filename)?;
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

struct Reactor<'t>
{
    result: String,
    problem: Option<char>,
    tried: &'t mut HashSet<char>,
}

impl<'t> Reactor<'t>
{
    pub fn new(tried: &'t mut HashSet<char>) -> Reactor<'t>
    {
        Reactor {
            result: String::new(),
            problem: None,
            tried,
        }
    }
}

fn annihilate(last: char, next: char) -> bool
{
    last.is_uppercase() ^ next.is_uppercase()
        && last.to_ascii_uppercase() == next.to_ascii_uppercase()
}

fn one_reaction(mut r: Reactor, next: char) -> Reactor
{
    let lower_next = next.to_ascii_lowercase();
    match r.problem {
        Some(p) if p == lower_next => {
            return r;
        }
        None if !r.tried.contains(&lower_next) => {
            println!("try problem: '{}'", lower_next);
            r.problem = Some(lower_next);
            r.tried.insert(lower_next);
            return r;
        }
        _ => {
            // next isn't the problem char, fall through
        }
    }

    match r.result.pop() {
        Some(last) => {
            if !annihilate(last, next) {
                r.result.push(last);
                r.result.push(next);
            }
        }
        None => {
            r.result.push(next)
        }
    }
    r
}

fn reactions(input: &str) -> (char, usize)
{
    let mut problems = HashSet::new();
    let mut worst_problem = ' ';
    let mut shortest_len = usize::MAX;
    loop {
        let reactor = Reactor::new(&mut problems);
        let result = input.chars().fold(reactor, one_reaction);
        match result.problem {
            Some(p) => {
                let prob_len = result.result.len();
                if prob_len < shortest_len {
                    println!(" worse problem '{}' length is {}", p, prob_len);
                    worst_problem = p;
                    shortest_len = prob_len;
                } else {
                    println!("lesser problem '{}' length is {}", p, prob_len);
                }
            }
            None => {
                return (worst_problem, shortest_len);
            }
        }
    }
}

fn main() -> io::Result<()>
{
    let full_input = read_input("input")?;
    let input = full_input.trim_end();
    println!("{}", input);
    println!("length: {}", input.len());
    let (worst_problem, result_len) = reactions(input);
    println!("worst problem: '{}'", worst_problem);
    println!("new length: {}", result_len);
    Ok(())
}
