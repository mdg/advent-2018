
use std::fs::File;
use std::io::{self, Read};


fn count_diff(a_str: &str, b_str: &str) -> Option<String>
{
    let mut same = String::new();
    let mut num_diff = 0;
    for (a, b) in a_str.chars().zip(b_str.chars()) {
        if a == b {
            same.push(a);
        } else {
            num_diff += 1;
            // same.push(' ');
            if num_diff > 1 {
                // println!("mismatch: {}", a_str);
                // println!("same:     {}", same);
                return None;
            }
        }
    }
    return Some(same);
}

fn main() -> io::Result<()>
{
    let mut f = File::open("input")?;
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)?;

    for l1 in buf.lines() {
        for l2 in buf.lines() {
            if l1 == l2 {
                continue;
            }

            // println!("{:?}", pair);
            let diff = count_diff(l1, l2);
            if diff.is_some() {
                println!("{}", diff.unwrap());
                return Ok(());
            }
        }
    }

    Ok(())
}
