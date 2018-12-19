
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


#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(Hash)]
struct Point
{
    pub x: i32,
    pub y: i32,
}

impl Point
{
    fn new(x: i32, y: i32) -> Point
    {
        Point { x, y }
    }

    fn manhattan_distance(a: Point, b: Point) -> i32
    {
        let x = i32::abs(a.x - b.x);
        let y = i32::abs(a.y - b.y);
        x + y
    }
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(Hash)]
struct GridPoint
{
    pub p: Point,
    pub nearest: Point,
    pub distance: i32,
}

impl GridPoint
{
    fn new(p: Point, nearest: Point) -> GridPoint
    {
        let distance = Point::manhattan_distance(p, nearest);
        GridPoint { p, nearest, distance }
    }
}


fn parse_point(line: &str) -> io::Result<Point>
{
    let mut parts = line.split(", ");
    let opt_x = parts.next();
    let opt_y = parts.next();
    match (opt_x, opt_y) {
        (Some(x_str), Some(y_str)) => {
            let x = i32::from_str_radix(x_str, 10).unwrap();
            let y = i32::from_str_radix(y_str, 10).unwrap();
            Ok(Point::new(x, y))
        }
        _ => {
            Err(io::Error::new(io::ErrorKind::InvalidData, line))
        }
    }
}

fn find_grid(points: &Vec<Point>) -> (i32, i32)
{
    let max_x = points.last().unwrap().x + 1;
    let max_y = points
        .iter()
        .fold(0, |acc, p| {
            cmp::max(acc, p.y)
        }) + 1;
    (max_x, max_y)
}

fn make_grid(bounds: (i32, i32), def: Point) -> Vec<Vec<GridPoint>>
{
    let mut grids = vec![];
    for y in 0..bounds.1 {
        let mut row = vec![];
        for x in 0..bounds.0 {
            let p = Point::new(x, y);
            let gp = GridPoint::new(p, def);
            println!("GridPoint: {:?}", gp);
            row.push(gp);
        }
        grids.push(row);
    }
    grids
}

fn assign_grid(grid_bounds: (i32, i32), points: &Vec<Point>) -> Vec<Vec<GridPoint>>
{
    let mut grid = make_grid(grid_bounds, *points.first().unwrap());
    let open_points: Vec<Point> = vec![];
    // let closed_points = vec![];
    for x in 0..grid_bounds.0 {
        for y in 0..grid_bounds.1 {
            // println!("{},{}", x, y);
        }
    }
    grid
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

    let grid = assign_grid(grid_bounds, &points);
    // println!("grids: {:?}", grid);

    Ok(())
}
