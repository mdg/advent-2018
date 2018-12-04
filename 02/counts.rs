
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};


fn count_letters(line: &str) -> (bool, bool)
{
    let mut counts = HashMap::new();
    // println!("{}", line);
    for c in line.chars() {
        let counter = counts.entry(c).or_insert(0);
        *counter += 1;
    }

    let mut has2 = false;
    let mut has3 = false;
    for v in counts.values() {
        match v {
            2 => {
                has2 = true;
            }
            3 => {
                has3 = true;
            }
            _ => {}
        }
    }
    (has2, has3)
}

fn main() -> io::Result<()>
{
    let mut f = File::open("input")?;
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)?;
    let mut count2 = 0;
    let mut count3 = 0;

    for line in buf.lines() {
        let (has2, has3) = count_letters(line);
        if has2 {
            count2 += 1;
        }
        if has3 {
            count3 += 1;
        }
    }

    let result = count2 * count3;
    println!("{}", result);
    Ok(())
}
