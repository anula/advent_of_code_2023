use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{min};
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
struct Range {
    source_start: i64,
    dest_start: i64,
    len: i64,
}

#[derive(Debug)]
struct Map {
    dest: String,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: HashMap<String, Map>,
}

impl Range {
    fn from_string(line: &str) -> Range {
        lazy_static! {
            static ref RANGE_RE : Regex = Regex::new(
                r"(?P<dest>\d+)\s+(?P<source>\d+)\s+(?P<len>\d+)"
            ).unwrap();
        }

        let caps = RANGE_RE.captures(line).unwrap();

        Range {
            source_start: caps.name("source").unwrap().as_str().parse().unwrap(),
            dest_start: caps.name("dest").unwrap().as_str().parse().unwrap(),
            len: caps.name("len").unwrap().as_str().parse().unwrap(),
        }
    }
}

impl Map {
    fn destination_for(&self, source: i64) -> i64 {
        for range in &self.ranges {
            if source >= range.source_start + range.len {
                continue;
            }
            if source < range.source_start {
                break;
            }
            let diff = source - range.source_start;
            return range.dest_start + diff;
        }
        return source;
    }
}

impl Almanac {

    fn follow_source_to_destination(&self, orig_val: i64, orig_source: &str, dest: &str) -> i64 {
        let mut val = orig_val;
        let mut source = orig_source;
        while source != dest {
            dprintln!("source: {}, val: {}", source, val);
            val = self.maps[source].destination_for(val);
            source = &self.maps[source].dest;
        }
        return val;
    }

    fn lowest_seed_destination(&self, dest: &str) -> i64 {
        let mut res = std::i64::MAX;
        for seed in &self.seeds {
            res = min(self.follow_source_to_destination(*seed, "seed", dest), res);
        }
        res
    }

    fn parse_seeds(line: &str) -> Vec<i64> {
        let mut seeds = Vec::<i64>::new();
        for seed in line.split_whitespace() {
            seeds.push(seed.parse::<i64>().unwrap());
        }
        return seeds;
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

        let seeds;
        {
            let seeds_line = lines.next().unwrap();
            let seeds_caps = SEEDS_RE.captures(&seeds_line).unwrap();
            seeds = Self::parse_seeds(seeds_caps.name("seeds").unwrap().as_str());
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
                ranges.push(Range::from_string(&range));
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
            seeds: seeds,
            maps: maps,
        }
    }

}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let almanac = Almanac::build(BufReader::new(input).lines().map(|l| l.unwrap()));
    dprintln!("Almanac: {:?}", almanac);

    writeln!(output, "{}", almanac.lowest_seed_destination("location")).unwrap();

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
            "35",
        );
    }
}
