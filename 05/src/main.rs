
use std::fs::File;
use std::io::{self, Read};

fn read_input(input_filename: &str) -> io::Result<String>
{
    let mut f = File::open(input_filename)?;
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn annihilate(last: char, next: char) -> bool
{
    last.is_uppercase() ^ next.is_uppercase()
        && last.to_ascii_uppercase() == next.to_ascii_uppercase()
}

fn one_reaction(mut acc: String, next: char) -> String
{
    match acc.pop() {
        Some(last) => {
            if !annihilate(last, next) {
                acc.push(last);
                acc.push(next);
            }
        }
        None => {
            acc.push(next)
        }
    }
    acc
}

fn reactions(input: &str) -> String
{
    input.chars().fold(String::new(), one_reaction)
}

fn main() -> io::Result<()>
{
    let full_input = read_input("input")?;
    let input = full_input.trim_end();
    println!("{}", input);
    println!("length: {}", input.len());
    let result = reactions(input);
    println!("new length: {}", result.len());
    Ok(())
}
