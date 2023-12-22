use std::cmp::{min, max};
use std::mem::swap;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;
use std::collections::HashMap;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Range(i64, i64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Range2D(Range, Range);

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Intervals2D {
    space: Vec<Vec<i64>>,
    brick_no: Vec<Vec<usize>>,
}

impl Intervals2D {
    fn new(size_x: usize, size_y: usize) -> Intervals2D {
        Intervals2D {
            space: vec![vec![0; size_x]; size_y],
            brick_no: vec![vec![0; size_x]; size_y],
        }
    }

    fn get_max(&self, range: Range2D) -> (i64, Vec<usize>) {
        let mut result_z = 0;
        let mut result_bricks = HashSet::new();

        let min_y = min(range.0.1, range.1.1);
        let min_x = min(range.0.0, range.1.0);
        let max_y = max(range.0.1, range.1.1);
        let max_x = max(range.0.0, range.1.0);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                result_z = max(result_z, self.space[y as usize][x as usize]); 
            }
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if result_z == self.space[y as usize][x as usize] {
                    result_bricks.insert(self.brick_no[y as usize][x as usize]);
                }
            }
        }
        (result_z, result_bricks.into_iter().collect())
    }

    fn set_range(&mut self, range: Range2D, val: i64, brick_no: usize) {
        let min_y = min(range.0.1, range.1.1);
        let min_x = min(range.0.0, range.1.0);
        let max_y = max(range.0.1, range.1.1);
        let max_x = max(range.0.0, range.1.0);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                self.space[y as usize][x as usize] = val;
                self.brick_no[y as usize][x as usize] = brick_no;
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.space.len(){
            for x in 0..self.space[y].len() {
                print!("{: >3}[{: >3}];", self.space[y][x], self.brick_no[y][x]);
            }
            println!();
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct XYZ {
    x: i64,
    y: i64,
    z: i64,
}

impl XYZ {
    fn new(x: i64, y: i64, z: i64) -> XYZ {
        XYZ { x, y, z }
    }

    fn from_str(s: &str) -> XYZ {
        let mut coords = s.split(',');
        XYZ {
            x: coords.next().unwrap().parse().unwrap(),
            y: coords.next().unwrap().parse().unwrap(),
            z: coords.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Brick(XYZ, XYZ);

impl Brick {
    fn footprint2D(&self) -> Range2D {
        Range2D(Range(self.0.x, self.0.y), Range(self.1.x, self.1.y))
    }

    fn arrange(&mut self) {
        if self.0.z > self.1.z {
            swap(&mut self.0, &mut self.1);
        }
    }

    fn low_z(&self) -> i64 {
        self.0.z
    }

    fn high_z(&self) -> i64 {
        self.1.z
    }

    fn put_on_level(&mut self, level: i64) {
        let h = self.1.z - self.0.z;
        self.0.z = level;
        self.1.z = level + h;
    }
}

#[derive(Debug, Clone)]
struct Cache {
}

#[derive(Debug, Clone, Hash)]
struct Jenga {
    bricks: Vec<Brick>,
    max_x: usize,
    max_y: usize,

    is_structural: Vec<bool>,
    supporting_directly: Vec<Vec<usize>>,
    lays_on: Vec<Vec<usize>>,
}

impl Jenga {
    fn new<I>(lines: I) -> Jenga
      where I: Iterator<Item = String>
    {
        let mut bricks = Vec::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for l in lines {
            let line = l.trim();
            let mut points = line.split('~');
            let point_a = XYZ::from_str(points.next().unwrap());
            let point_b = XYZ::from_str(points.next().unwrap());
            let mut brick = Brick(point_a, point_b);
            brick.arrange();
            bricks.push(brick);

            max_x = max(max_x, max(point_a.x, point_b.x) as usize);
            max_y = max(max_y, max(point_a.y, point_b.y) as usize);
        }
        // base brick
        bricks.push(Brick(XYZ::new(0, 0, 0), XYZ::new(max_x as i64, max_y as i64, 0)));
        bricks.sort_by_key(|b| (b.0.z, b.1.z));

        Jenga {
            is_structural: vec![false; bricks.len()],
            supporting_directly: vec![vec![]; bricks.len()],
            lays_on: vec![vec![]; bricks.len()],

            bricks,
            max_x,
            max_y,
        }
    }

    fn fall_bricks(&mut self) {
        let mut intervals = Intervals2D::new(self.max_x + 1, self.max_y + 1);


        for (no, brick) in &mut self.bricks.iter_mut().enumerate().skip(1) {
            let (max_ground, mut holding_bricks) = intervals.get_max(brick.footprint2D());
            brick.put_on_level(max_ground + 1);
            if holding_bricks.len() == 1 {
                self.is_structural[holding_bricks[0]] = true;
            }
            for &hb in &holding_bricks {
                self.supporting_directly[hb].push(no);
            }
            self.lays_on[no].append(&mut holding_bricks);
            intervals.set_range(brick.footprint2D(), brick.high_z(), no);
        }
    }

    fn is_touching_ground(&self, brick_no: usize, missing_brick: usize) -> bool {
        if brick_no == missing_brick {
            return false;
        }
        if brick_no == 0 {
            return true;
        }
        for &lo in &self.lays_on[brick_no] {
            if self.is_touching_ground(lo, missing_brick) {
                return true;
            }
        }

        false
    }

    fn count_total_falls(&self) -> usize {
        let mut result = 0;
        for i in 1..self.is_structural.len() {
            if !self.is_structural[i] {
                continue;
            }
            let falling = (1..self.bricks.len()).map(|b| self.is_touching_ground(b, i)).filter(|x| !x).count();
            result += falling - 1;
        }
        result
    }

    #[allow(dead_code)]
    fn print_bricks(&self) {
        println!("Bricks:");
        for b in &self.bricks {
            println!("  {:?}", b);
        }
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {

    let mut jenga = Jenga::new(BufReader::new(input).lines().map(|l| l.unwrap()));
    dprintln!("Jenga: {:?}", jenga);
    jenga.fall_bricks();
    dprintln!("Jenga fallen: {:?}", jenga);


    writeln!(output, "{}", jenga.count_total_falls()).unwrap();
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
            "1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9",
            "7",
        );
    }

    #[test]
    fn mine() {
        test_ignore_whitespaces(
            "0,0,1~0,2,1
            0,1,3~2,1,3
            2,1,9~2,2,9",
            "3",
        );
    }

    #[test]
    fn mine2() {
        test_ignore_whitespaces(
            "0,0,1~0,2,1
            0,1,3~2,1,3
            2,1,3~2,1,5
            2,1,9~2,2,9",
            "6",
        );
    }

    #[test]
    fn two_legs() {
        test_ignore_whitespaces(
            "0,0,1~0,0,5
            0,1,1~0,1,5
            0,0,7~0,1,7
            0,1,9~0,1,10",
            "1",
        );
    }

    #[test]
    fn one_leg() {
        test_ignore_whitespaces(
            "0,1,1~0,1,5
            0,0,6~0,2,6
            0,0,7~0,0,8
            0,2,7~0,2,8
            1,1,1~1,1,7
            0,1,8~1,1,8
            0,0,9~0,2,9",
            "6",
        );
    }
}
