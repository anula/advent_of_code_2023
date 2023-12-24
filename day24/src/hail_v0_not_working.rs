//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[allow(unused_macros)]
macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct XYZ {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl XYZ {
    const fn new(x: f64, y: f64, z: f64) -> XYZ { XYZ {x, y, z} }

    fn add(&self, other: &XYZ) -> XYZ {
        XYZ { x: self.x + other.x, y: self.y + other.y , z: self.z + other.z }
    }
    fn sub(&self, other: &XYZ) -> XYZ {
        XYZ { x: self.x - other.x, y: self.y - other.y , z: self.z - other.z }
    }
    fn mul(&self, scalar: f64) -> XYZ {
        XYZ { x: self.x * scalar, y: self.y * scalar , z: self.z * scalar }
    }

    fn in_square_2d(&self, min_coord: f64, max_coord: f64) -> bool {
        self.x >= min_coord && self.x <= max_coord &&
            self.y >= min_coord && self.y <= max_coord
    }

    fn in_rect_2d(&self, rect: &Rectangle) -> bool {
        self.x >= rect.min_x && self.x <= rect.max_x &&
            self.y >= rect.min_y && self.y <= rect.max_y
    }

    fn vector_product_2d(&self, other: XYZ) -> f64 {
        other.x * self.y - self.x * other.y
    }

    fn from_str(st: &str) -> XYZ {
        let parts: Vec<_>  = st.split(", ").collect();
        XYZ {
            x: parts[0].trim().parse::<f64>().unwrap(),
            y: parts[1].trim().parse::<f64>().unwrap(),
            z: parts[2].trim().parse::<f64>().unwrap(),
        }
    }
}

impl Hash for XYZ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:?}", self).hash(state)
    }
}

impl Eq for XYZ {}

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

    fn at_time(&self, t: f64) -> XYZ {
        self.start.add(&self.velocity.mul(t))
    }

    fn at_x(&self, x: f64) -> Option<XYZ> {
        let t = (x - self.start.x) / self.velocity.x;
        if t >= 0. {
            Some(self.at_time(t))
        } else {
            None
        }
    }

    fn at_y(&self, y: f64) -> Option<XYZ> {
        let t = (y - self.start.y) / self.velocity.y;
        if t >= 0. {
            Some(self.at_time(t))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn at_z(&self, z: f64) -> Option<XYZ> {
        let t = (z - self.start.z) / self.velocity.z;
        if t >= 0. {
            Some(self.at_time(t))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Rectangle {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

#[derive(Debug, PartialEq, Clone)]
struct CutThroughRec {
    cut_points: [XYZ; 2],

    rect: Rectangle,
}

impl CutThroughRec {
    fn try_cutting(ray: &Ray, min: f64, max: f64) -> Option<CutThroughRec> {
        let mut rect = Rectangle {
            min_x: min,
            min_y: min,
            max_x: max,
            max_y: max,
        };
        if ray.start.in_square_2d(min, max) {
            if ray.velocity.x > 0. {
                rect.min_x = ray.start.x;
            } else {
                rect.max_x = ray.start.x;
            }
            if ray.velocity.y > 0. {
                rect.min_y = ray.start.y;
            } else {
                rect.max_y = ray.start.y;
            }
        }
        let cuts: Vec<XYZ> = Self::cut_points(ray, &rect).into_iter().collect();
        println!("Cuts for {:?}, are: {:?}", ray, cuts);
        println!("with rect: {:?}", rect);

        if cuts.len() == 2 {
            Some(CutThroughRec {
                cut_points: [cuts[0], cuts[1]],
                rect,
            })
        } else {
            None
        }
    }

    fn cut_points(ray: &Ray, rect: &Rectangle) -> HashSet<XYZ> {
        let potential_points = [
            ray.at_x(rect.min_x),
            ray.at_x(rect.max_x),
            ray.at_y(rect.min_y),
            ray.at_y(rect.max_y),
        ];
        println!("pot points: {:?}", potential_points);
        potential_points.into_iter()
            .filter(|o| o.is_some()).map(|o| o.unwrap()).filter(|p| p.in_rect_2d(rect)).collect()
    }



    fn crosses_the_cut(&self, ray: &Ray) -> bool {
        println!("Checking cross: {:?}, for ray: {:?}", self, ray);
        let mut other_cuts = Self::cut_points(ray, &self.rect);
        println!(" - potential cuts: {:?}", other_cuts);
        if ray.start.in_rect_2d(&self.rect) {
            other_cuts.insert(ray.start);
        }
        println!(" - potential cuts: {:?}", other_cuts);
        if other_cuts.len() < 2 {
            return false;
        }
        if other_cuts.len() > 2 {
            panic!("There should be max 2 cuts");
        }
        let other_cuts: Vec<XYZ> = other_cuts.into_iter().collect();
        let base_point = self.cut_points[0];
        let dir1 = self.cut_points[1].sub(&base_point).vector_product_2d(other_cuts[0].sub(&base_point));
        let dir2 = self.cut_points[1].sub(&base_point).vector_product_2d(other_cuts[1].sub(&base_point));

        println!(" - did we find a cut: {:?}", dir1 * dir2 < 0.);
        dir1 * dir2 < 0.0
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Solution {
    rays: Vec<Ray>,
}

impl Solution {
    fn collisions_inside(&self, min_coord: f64, max_coord: f64) -> i64 {
        let mut collisions = 0;

        let cuts: Vec<_> =
            self.rays.iter().map(|r| CutThroughRec::try_cutting(r, min_coord, max_coord)).collect();
        println!("Cuts: {:?}", cuts);
        for i in 0..(cuts.len() - 1) {
            if let Some(base_cut) = &cuts[i] {
                for j in (i + 1)..cuts.len() {
                    println!("Checking {} with {}", i, j);
                    if base_cut.crosses_the_cut(&self.rays[j]) {
                        println!("no {} crosses {}", i, j);
                        collisions += 1;
                    }
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

    writeln!(output, "{}", sol.collisions_inside(200000000000000., 400000000000000.)).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_solution(input: &str, min: f64, max: f64, expected_out: i64) {
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
            7., 27.,
            2,
        );
    }
}
