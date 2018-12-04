
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};


fn main() -> io::Result<()>
{
    let mut f = File::open("input")?;
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)?;
    let mut sets = HashSet::new();
    let mut freq = 0;
    loop {
        for line in buf.lines() {
            // println!("{:?}", line);
            let change = i32::from_str_radix(line, 10).unwrap();
            freq += change;
            if sets.contains(&freq) {
                println!("{}", freq);
                return Ok(());
            }
            sets.insert(freq);
        }
    }
}
