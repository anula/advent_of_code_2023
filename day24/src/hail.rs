//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};

#[allow(unused_macros)]
macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Copy, Clone)]
struct XYZ {
    x: i128,
    y: i128,
    z: i128,
}

#[allow(dead_code)]
impl XYZ {
    const fn new(x: i128, y: i128, z: i128) -> XYZ { XYZ {x, y, z} }

    fn add(&self, other: &XYZ) -> XYZ {
        XYZ { x: self.x + other.x, y: self.y + other.y , z: self.z + other.z }
    }
    fn sub(&self, other: &XYZ) -> XYZ {
        XYZ { x: self.x - other.x, y: self.y - other.y , z: self.z - other.z }
    }
    fn mul(&self, scalar: i128) -> XYZ {
        XYZ { x: self.x * scalar, y: self.y * scalar , z: self.z * scalar }
    }

    fn in_rect_2d(&self, rect: &Rectangle) -> bool {
        self.x >= rect.min_x && self.x <= rect.max_x &&
            self.y >= rect.min_y && self.y <= rect.max_y
    }

    fn vector_product_2d(&self, other: &XYZ) -> i128 {
        other.x * self.y - self.x * other.y
    }

    fn from_str(st: &str) -> XYZ {
        let parts: Vec<_>  = st.split(", ").collect();
        XYZ {
            x: parts[0].trim().parse::<i128>().unwrap(),
            y: parts[1].trim().parse::<i128>().unwrap(),
            z: parts[2].trim().parse::<i128>().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Hash)]
struct Ray {
    start: XYZ,
    velocity: XYZ,
}

impl Ray {
    fn from_line(line: &str) -> Ray {
        let mut parts = line.split(" @ ");
        Ray {
            start: XYZ::from_str(parts.next().unwrap()),
            velocity: XYZ::from_str(parts.next().unwrap()),
        }
    }

    fn crosses_with(&self, other: &Ray, rect: &Rectangle) -> bool {
        let mut vec_pro = self.velocity.vector_product_2d(&other.velocity);
        // 1. Check if parallel.
        if vec_pro == 0 {
            return false;
        }
        
        // 2. Check if t > 0
        let mut t_self_numerator = (self.start.x - other.start.x) * other.velocity.y +
            (other.start.y - self.start.y) * other.velocity.x;
        let mut t_other_numerator = (self.start.x - other.start.x) * self.velocity.y +
            (other.start.y - self.start.y) * self.velocity.x;

        if vec_pro > 0 {
            if t_self_numerator < 0 || t_other_numerator < 0 {
                return false;
            }
        } else {
            if t_self_numerator > 0 || t_other_numerator > 0 {
                return false;
            }
        }

        // 3. Check if in rectangle
        if vec_pro < 0 {
            vec_pro *= -1;
            t_self_numerator *= -1;
        }

        if t_self_numerator * self.velocity.x < (rect.min_x - self.start.x) * vec_pro {
            return false;
        }

        if t_self_numerator * self.velocity.x > (rect.max_x - self.start.x) * vec_pro {
            return false;
        }

        if t_self_numerator * self.velocity.y < (rect.min_y - self.start.y) * vec_pro {
            return false;
        }

        if t_self_numerator * self.velocity.y > (rect.max_y - self.start.y) * vec_pro {
            return false;
        }

        true
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Rectangle {
    min_x: i128,
    min_y: i128,
    max_x: i128,
    max_y: i128,
}

impl Rectangle {
    fn new(min_coord: i128, max_coord: i128) -> Rectangle {
        Rectangle {
            min_x: min_coord,
            min_y: min_coord,
            max_x: max_coord,
            max_y: max_coord,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Solution {
    rays: Vec<Ray>,
}

impl Solution {
    fn collisions_inside(&self, min_coord: i128, max_coord: i128) -> i128 {
        let mut collisions = 0;
        let rect = Rectangle::new(min_coord, max_coord);

        for i in 0..(self.rays.len() - 1) {
            for j in (i + 1)..self.rays.len() {
                if self.rays[i].crosses_with(&self.rays[j], &rect) {
                    collisions += 1;
                }
            }
        }
        collisions
    }
}


fn parse_input<R: BufRead>(input: R) -> Solution {
    let mut rays = Vec::new();
    for line in BufReader::new(input).lines().map(|l| l.unwrap()) {
        rays.push(Ray::from_line(line.trim()));
    }
    dprintln!("Rays: {:?}", rays);
    Solution {
        rays,
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let sol = parse_input(input);

    writeln!(output, "{}", sol.collisions_inside(200000000000000, 400000000000000)).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_solution(input: &str, min: i128, max: i128, expected_out: i128) {
        let sol = parse_input(input.as_bytes());
        assert_eq!(sol.collisions_inside(min, max), expected_out);
    }

    #[test]
    fn sample() {
        test_solution(
            "19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3",
            7, 27,
            2,
        );
    }
}
