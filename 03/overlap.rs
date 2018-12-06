
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};
use std::default::Default;


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
    x: i32,
    y: i32,
}

impl Point
{
    pub fn new(x: i32, y: i32) -> Point
    {
        Point{ x, y }
    }
}

impl Default for Point
{
    fn default() -> Point
    {
        Point{ x: 0, y: 0 }
    }
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
struct Rect<'a>
{
    id: &'a str,
    pos: Point,
    size: Point,
}

impl<'a> Rect<'a>
{
    pub fn new(id: &'a str, pos: Point, size: Point) -> Rect<'a>
    {
        Rect {
            id,
            pos,
            size,
        }
    }

    pub fn right(&self) -> i32
    {
        self.pos.x + self.size.x - 1
    }

    pub fn bottom(&self) -> i32
    {
        self.pos.y + self.size.y - 1
    }

    pub fn area(&self) -> i32
    {
        self.size.x * self.size.y
    }

    pub fn iter(&self) -> RectIter<'a>
    {
        RectIter::new(*self)
    }
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
struct RectIter<'a>
{
    bounds: Rect<'a>,
    cur: Point,
}

impl<'a> RectIter<'a>
{
    pub fn new(r: Rect<'a>) -> RectIter<'a>
    {
        RectIter{
            bounds: r,
            cur: r.pos,
        }
    }
}

impl<'a> Iterator for RectIter<'a>
{
    type Item = Point;

    fn next(&mut self) -> Option<Point>
    {
        if self.cur.y > self.bounds.bottom() {
            return None;
        }
        let result = self.cur;
        self.cur.x += 1;
        if self.cur.x > self.bounds.right() {
            self.cur.x = self.bounds.pos.x;
            self.cur.y += 1;
        }
        Some(result)
    }
}

fn read_input() -> io::Result<String>
{
    let mut f = File::open("input")?;
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn read_rect(input: &str) -> Rect
{
    let mut id_split = input.split(" @ ");
    let id = id_split.next().unwrap().get(1..).unwrap();
    let mut rect_split = id_split.next().unwrap().split(": ");
    let pos_str: Vec<&str> = rect_split.next().unwrap().split(',').collect();
    let size_str: Vec<&str> = rect_split.next().unwrap().split('x').collect();

    let left = i32::from_str_radix(pos_str.get(0).unwrap(), 10).unwrap();
    let top = i32::from_str_radix(pos_str.get(1).unwrap(), 10).unwrap();
    let width = i32::from_str_radix(size_str.get(0).unwrap(), 10).unwrap();
    let height = i32::from_str_radix(size_str.get(1).unwrap(), 10).unwrap();
    Rect::new(id, Point::new(left, top), Point::new(width, height))
}

fn main() -> io::Result<()>
{
    let input = read_input()?;
    let mut ids = HashSet::new();
    let mut fabric = HashMap::new();
    let mut noverlaps = 0;
    for line in input.lines() {
        let rect = read_rect(line);
        ids.insert(rect.id);
        // println!("{} -> {}", line, area);
        for p in rect.iter() {
            // println!("mark {:?}", p);
            let overlap_claims: &mut Vec<&str> =
                fabric.entry(p).or_insert(Vec::new());
            overlap_claims.push(rect.id);
            if overlap_claims.len() == 2 {
                // println!("overlap {:?}", p);
                noverlaps += 1;
            }
        }
    }
    // println!("{:?}", fabric);
    let overlaps: Vec<(Point, Vec<&str>)> = fabric
        .into_iter()
        .filter(|si| {
            si.1.len() > 1
        })
        .collect();
    for o in overlaps.iter() {
        for id in o.1.iter() {
            ids.remove(id);
        }
    }
    println!("{}", overlaps.len());
    println!("{}", noverlaps);
    println!("ids len: {}", ids.len());
    println!("first id: {}", ids.iter().next().unwrap());
    Ok(())
}
