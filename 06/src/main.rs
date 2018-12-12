
use std::cmp;
use std::fs::File;
use std::io::{self, Read};


fn read_input(input_filename: &str) -> io::Result<String>
{
    let mut f = File::open(input_filename)?;
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn parse_point(line: &str) -> io::Result<(i16, i16)>
{
    let mut parts = line.split(", ");
    let opt_x = parts.next();
    let opt_y = parts.next();
    match (opt_x, opt_y) {
        (Some(x_str), Some(y_str)) => {
            let x = i16::from_str_radix(x_str, 10).unwrap();
            let y = i16::from_str_radix(y_str, 10).unwrap();
            Ok((x, y))
        }
        _ => {
            Err(io::Error::new(io::ErrorKind::InvalidData, line))
        }
    }
}

fn find_grid(points: &Vec<(i16, i16)>) -> (i16, i16)
{
    let max_x = points.last().unwrap().0 + 1;
    let max_y = points
        .iter()
        .fold(0, |acc, p| {
            cmp::max(acc, p.1)
        }) + 1;
    (max_x, max_y)
}

fn make_grid(points: &Vec<(i16, i16)>, def: (i16, i16)) -> Vec<Vec<(i16, i16)>>
{
    let max_x: usize = points.last().unwrap().0 as usize + 1;
    let max_y: usize = points
        .iter()
        .fold(0, |acc, p| {
            cmp::max(acc, p.1)
        }) as usize + 1;
    println!("grid: {}, {}", max_x, max_y);
    vec![vec![def; max_x]; max_y]
}

fn assign_grid(grid_bounds: (i16, i16), points: &Vec<(i16, i16)>) -> Vec<Vec<(i16, i16)>>
{
    j
}

fn main() -> io::Result<()>
{
    let input = read_input("sorted_input")?;
    let mut points = vec![];
    for line in input.lines() {
        let p = parse_point(line)?;
        points.push(p);
    }
    println!("{:?}", points);

    let grid_bounds = find_grid(&points);
    println!("grid bounds: {:?}", grid_bounds);
    let grid = make_grid(&points, *points.first().unwrap());

    Ok(())
}
