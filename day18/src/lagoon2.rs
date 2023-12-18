//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use regex::Regex;
use lazy_static::lazy_static;

#[allow(unused_macros)]
macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct XY {
    x: i64,
    y: i64,
}

impl XY {
    const fn new(x: i64, y: i64) -> XY { XY {x, y} }

    const fn add(&self, other: &XY) -> XY { XY { x: self.x + other.x, y: self.y + other.y } }
    const fn mul(&self, scalar: i64) -> XY { XY { x: self.x * scalar, y: self.y * scalar } }

    fn dir(c: &str) -> XY {
        match c {
            "3" => XY::new(0, -1),
            "0" => XY::new(1, 0),
            "1" => XY::new(0, 1),
            "2" => XY::new(-1, 0),
            _ => panic!("No such dir!"),
        }
    }

    fn len_dir(c: &str, len: i64) -> XY { Self::dir(c).mul(len) }
}

#[derive(Debug)]
struct Intervals {
    ints: Vec<(i64, i64)>,
}

impl Intervals {
    fn empty() -> Intervals {
        Intervals {
            ints: Vec::new(),
        }
    }

    fn is_contained(&self, range: (i64, i64)) -> bool {
        for &(start, end) in &self.ints {
            if range.0 >= start && range.1 < end {
                return true;
            }
        }
        false
    }

    fn total_len(&self) -> i64 {
        let mut len = 0;

        for (start, end) in &self.ints {
            len += end - start;
        }
        len
    }

    fn add(&mut self, range: (i64, i64)) {
        if self.is_contained(range) { return; }
        self.ints.push((range.0, range.1 + 1));
        self.compress();
    }

    fn remove(&mut self, range: (i64, i64)) -> i64 {
        if !self.is_contained(range) { return 0; }
        let mut removed = 0;

        for i in 0..self.ints.len() {
            if range.0 >= self.ints[i].0 && range.1 < self.ints[i].1 {
                let curr_start = self.ints[i].0;
                let curr_end = self.ints[i].1;
                let start_remove =
                    if range.0 == curr_start {
                        range.0
                    } else {
                        range.0 + 1
                    };
                let end_remove =
                    if range.1 == curr_end - 1 {
                        curr_end
                    } else {
                        range.1
                    };
                removed = end_remove - start_remove;
                self.ints[i].1 = start_remove;
                self.ints.push((end_remove, curr_end));
                break;
            }
        }
        self.compress();
        removed
    }

    fn compress(&mut self) {
        let mut new_ints = Vec::new();
        self.ints.sort();
        new_ints.push(self.ints[0]);
        for i in 1..self.ints.len() {
            let prev = new_ints.last_mut().unwrap();
            let curr = self.ints[i];
            if curr.0 == curr.1 { continue; }
            if prev.1 >= curr.0 {
                prev.1 = curr.1;
            } else {
                new_ints.push(curr);
            }
        }
        if new_ints[0].0 == new_ints[0].1 {
            new_ints.remove(0);
        }
        self.ints = new_ints;
    }
}


#[derive(Debug)]
struct Lagoon {
    corners: Vec<XY>,
}

impl Lagoon {

    fn from_input<I>(lines: I) -> Lagoon
        where I: Iterator<Item = String>
    {
        lazy_static! {
            static ref INS_RE: Regex = Regex::new(
                r"(?P<dir>[URDL]) (?P<len>\d+) \(#(?P<color>.*)\)"
            ).unwrap();
        }

        let mut corners = Vec::new();

        let mut curr = XY::new(0, 0);
        corners.push(curr);

        for l in lines {
            let line = l.trim();
            let caps = INS_RE.captures(line).unwrap();
            let color = caps.name("color").unwrap().as_str();
            let len: i64 = i64::from_str_radix(&color[0..5], 16).unwrap();
            curr = curr.add(&XY::len_dir(&color[5..6], len));
            corners.push(curr);
        }

        if corners[0] == *corners.last().unwrap() {
            corners.pop();
        }

        Lagoon {
            corners,
        }
    }

    fn size(&mut self) -> i64 {
        let mut count = 0;
        let mut inside_intervals = Intervals::empty();

        // Sort by rows
        self.corners.sort_by_key(|p| (p.y, p.x));
        let mut prev_row = self.corners[0].y;

        let mut i = 0;
        while i < self.corners.len() {
            let row = self.corners[i].y;
            let to_add = ((row - prev_row).abs()) * inside_intervals.total_len();
            dprintln!("--For rows: {} - {}", prev_row, row);
            dprintln!("Intervals were: {:?}", inside_intervals);
            dprintln!("Thus we add: {:?}", to_add);
            count += to_add;
            while i < self.corners.len() && self.corners[i].y == row {
                let first = self.corners[i];
                if i + 1 >= self.corners.len() {
                    panic!("There were odd num of corners at end of row: {}", row);
                }
                let second = self.corners[i+1];
                if second.y != row {
                    panic!("There were odd num of corners in row: {}", row);
                }

                let new_range = (first.x, second.x);
                if inside_intervals.is_contained(new_range) {
                    count += inside_intervals.remove(new_range);
                } else {
                    inside_intervals.add(new_range);
                }
                i += 2;
            }
            prev_row = row;
        }

        count
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut lagoon = Lagoon::from_input(lines);
    dprintln!("Lagoon: {:?}", lagoon);

    writeln!(output, "{}", lagoon.size()).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ignore_whitespaces(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        let actual_out_str = String::from_utf8(actual_out).unwrap();
        let actual_outs = actual_out_str.split_whitespace().collect::<Vec<&str>>();
        let expected_outs = output.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(actual_outs, expected_outs);
    }

    #[test]
    fn sample() {
        test_ignore_whitespaces(
            "R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)",
            "952408144115",
        );
    }
}
