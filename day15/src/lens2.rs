//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use regex::Regex;
use lazy_static::lazy_static;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

fn hash(acc: i64, c: char) -> i64 { ((acc + (c as i64)) * 17) % 256 }

fn compute_hash(word: &str) -> usize { word.chars().fold(0, hash) as usize }

#[derive(Debug)]
struct Boxy {
    number: usize,
    lenses_list: Vec<i64>,
    lenses_pos: Vec<String>,
}

impl Boxy {
    fn new(number: usize) -> Boxy {
        Boxy {
            number,
            lenses_list: Vec::new(),
            lenses_pos: Vec::new(),
        }
    }

    fn insert(&mut self, label: &str, val: i64) {
        if let Some(pos) = self.lenses_pos.iter().position(|s| s == label) {
            self.lenses_list[pos] = val;
        } else {
            self.lenses_list.push(val);
            self.lenses_pos.push(String::from(label));
        }
    }

    fn delete(&mut self, label: &str) {
        if let Some(pos) = self.lenses_pos.iter().position(|s| s == label) {
            self.lenses_list.remove(pos);
            self.lenses_pos.remove(pos);
        }
    }

    fn power(&self) -> i64 {
        let mut pow = 0;
        for (i, val) in self.lenses_list.iter().enumerate() {
            let slot = i + 1;
            pow += (self.number as i64) * slot as i64 * val;
        }
        pow
    }

    fn is_empty(&self) -> bool {
        self.lenses_list.is_empty()
    }
}

#[derive(Debug)]
struct Boxes {
    boxes: Vec<Boxy>,
}

impl Boxes {
    fn new() -> Boxes {
        let mut boxes = Vec::new();

        for i in 0..=255 {
            boxes.push(Boxy::new(i + 1));
        }

        Boxes {
            boxes,
        }
    }

    fn insert(&mut self, label: &str, val: i64) {
        dprintln!("insert({}, {})", label, val);
        let box_pos = compute_hash(label);
        self.boxes[box_pos].insert(label, val);
    }

    fn delete(&mut self, label: &str) {
        dprintln!("delete({})", label);
        let box_pos = compute_hash(label);
        self.boxes[box_pos].delete(label);
    }

    fn apply(&mut self, action: &str) {
        lazy_static! {
            static ref ACTION_RE : Regex = Regex::new(
                r"(?P<label>.+)(?P<action>=|-)(?P<val>\d+)?"
            ).unwrap();
        }
        dprintln!("Action: {}", action);

        let caps = ACTION_RE.captures(action).unwrap();

        let label = caps.name("label").unwrap().as_str();
        let action = caps.name("action").unwrap().as_str();

        match action {
            "=" => {
                let val: i64 = caps.name("val").unwrap().as_str().parse().unwrap();
                self.insert(label, val);
            },
            "-" => self.delete(label),
            _ => panic!("Wrong action"),
        }
    }

    fn power(&self) -> i64 {
        self.boxes.iter().map(|b| b.power()).sum()
    }

    fn dprint_non_empty(&self) {
        for b in &self.boxes {
            if !b.is_empty() {
                dprintln!("{:?}", b);
            }
        }
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {

    let input = BufReader::new(input).lines().map(|l| l.unwrap()).next().unwrap();

    let mut boxes = Boxes::new();

    for a in input.split(',') {
        boxes.apply(a);
    }

    dprintln!("Boxes: ");
    boxes.dprint_non_empty();

    writeln!(output, "{}", boxes.power()).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_exact(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        assert_eq!(String::from_utf8(actual_out).unwrap(), output);
    }

    fn test_ignore_whitespaces(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        let actual_out_str = String::from_utf8(actual_out).unwrap();
        let actual_outs = actual_out_str.split_whitespace().collect::<Vec<&str>>();
        let expected_outs = output.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(actual_outs, expected_outs);
    }

    #[test]
    fn hash() {
        assert_eq!(compute_hash("HASH"), 52);
    }

    #[test]
    fn sample() {
        test_ignore_whitespaces(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
            "145",
        );
    }
}
