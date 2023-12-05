use lazy_static::lazy_static;
use regex::Regex;
//use std::cmp::{min};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug)]
struct RangeMapping {
    source_start: i64,
    dest_start: i64,
    len: i64,
}

#[derive(Debug)]
struct Range {
    start: i64,
    len: i64,
}


#[derive(Debug)]
struct Ranges {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Map {
    dest: String,
    ranges: Vec<RangeMapping>,
}

#[derive(Debug)]
struct Almanac {
    seeds_ranges: Ranges,
    maps: HashMap<String, Map>,
}

impl RangeMapping {
    fn from_string(line: &str) -> RangeMapping {
        lazy_static! {
            static ref RANGE_RE : Regex = Regex::new(
                r"(?P<dest>\d+)\s+(?P<source>\d+)\s+(?P<len>\d+)"
            ).unwrap();
        }

        let caps = RANGE_RE.captures(line).unwrap();

        RangeMapping {
            source_start: caps.name("source").unwrap().as_str().parse().unwrap(),
            dest_start: caps.name("dest").unwrap().as_str().parse().unwrap(),
            len: caps.name("len").unwrap().as_str().parse().unwrap(),
        }
    }

    fn source_range(&self) -> Range { Range { start: self.source_start, len: self.len }}

    fn diff(&self) -> i64 { self.dest_start - self.source_start }
}

impl Range {
    fn new(from: i64, to_exclusive: i64) -> Range {
        Range {
            start: from,
            len: to_exclusive - from,
        }
    }

    fn contains(&self, x: i64) -> bool { (x >= self.start) && (x < self.start + self.len) }
    fn starts_with(&self, x: i64) -> bool { x == self.start }
    fn is_within(&self, other: &Range) -> bool {
        other.start <= self.start && other.last_item() >= self.last_item()
    }
    fn is_fully_greater_than(&self, other: &Range) -> bool {
        other.start + other.len <= self.start
    }

    fn last_item(&self) -> i64 { self.start + self.len - 1 }

    // Returns the higher range as new one
    fn split_at(&mut self, split: i64) -> Range {
        if !self.contains(split) {
            panic!("Trying to split outside of range");
        }

        let to_exclusive = self.start + self.len;

        self.len = split - self.start;

        Range::new(split, to_exclusive)
    }
}

impl Ranges {

    fn sort(&mut self) {
        self.ranges.sort_by_key(|r| r.start);
    }

    fn lowest(&self) -> i64 { self.ranges[0].start }

    fn split_ranges_on(&mut self, new_start: i64) {
        let mut to_split = None;
        for i in 0..self.ranges.len() {
            let range = &self.ranges[i];
            if range.contains(new_start) {
                to_split = Some(i);
                break;
            }
        }

        if let Some(idx) = to_split {
            let range = &mut self.ranges[idx];

            if range.starts_with(new_start) {
                return
            }

            let new_range = range.split_at(new_start);
            self.ranges.push(new_range);
            self.sort();
        }
    }

    fn transform(&mut self, map: &Map) {
        for mapping in &map.ranges {
            self.split_ranges_on(mapping.source_start);
            self.split_ranges_on(mapping.source_start + mapping.len);
        }

        let mut i = 0;
        for mapping in &map.ranges {
            while i < self.ranges.len() && !self.ranges[i].is_fully_greater_than(&mapping.source_range()) {
                let range = &mut self.ranges[i];
                if range.is_within(&mapping.source_range()) {
                    range.start += mapping.diff();
                }
                i += 1;
            }

            if i >= self.ranges.len() {
                break;
            }
        }

        self.sort();
    }
}

impl Almanac {

    fn parse_seeds_ranges(line: &str) -> Vec<Range> {
        let mut seeds = Vec::<Range>::new();
        let mut seed_iter = line.split_whitespace();
        while let Some(seed) = seed_iter.next() {
            let len = seed_iter.next().unwrap();
            seeds.push(Range {
                start: seed.parse().unwrap(),
                len: len.parse().unwrap(),
            });
        }
        return seeds;
    }

    fn transform_from_to(&mut self, source: &str, dest: &str) {
        let mut curr_from = source;

        while curr_from != dest {
            self.seeds_ranges.transform(&self.maps[curr_from]);
            curr_from = &self.maps[curr_from].dest;
        }
    }

    fn lowest_in_range(&self) -> i64 {
        self.seeds_ranges.lowest()
    }

    fn build<I>(mut lines: I) -> Almanac
        where I: Iterator<Item = String>
    {
        lazy_static! {
            static ref SEEDS_RE : Regex = Regex::new(
                r"seeds: (?P<seeds>.*)"
            ).unwrap();
            static ref MAP_RE : Regex = Regex::new(
                r"(?P<source>[a-z]+)-to-(?P<dest>[a-z]+) map:"
            ).unwrap();
        }

        let mut seeds;
        {
            let seeds_line = lines.next().unwrap();
            let seeds_caps = SEEDS_RE.captures(&seeds_line).unwrap();
            seeds = Self::parse_seeds_ranges(seeds_caps.name("seeds").unwrap().as_str());
            seeds.sort_by_key(|r| r.start);
        }

        // empty line
        let _ = lines.next();

        let mut maps = HashMap::new();

        while let Some(map) = lines.next() {
            let map_caps = MAP_RE.captures(&map).unwrap();
            let source = map_caps.name("source").unwrap().as_str();
            let dest = map_caps.name("dest").unwrap().as_str();

            let mut ranges = Vec::new();
            while let Some(range) = lines.next() {
                if range.trim() == "" {
                    break;
                }
                ranges.push(RangeMapping::from_string(&range));
            }

            ranges.sort_by_key(|r| r.source_start);
            maps.insert(
                source.to_string(),
                Map {
                    dest: dest.to_string(),
                    ranges: ranges,
                }
            );
        }

        Almanac {
            seeds_ranges: Ranges { ranges: seeds },
            maps: maps,
        }
    }

}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut almanac = Almanac::build(BufReader::new(input).lines().map(|l| l.unwrap()));
    dprintln!("Almanac: {:?}", almanac);

    almanac.transform_from_to("seed", "location");

    writeln!(output, "{}", almanac.lowest_in_range()).unwrap();

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
    fn sample() {
        test_ignore_whitespaces(
            "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4",
            "46",
        );
    }
}
