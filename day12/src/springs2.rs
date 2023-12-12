//use std::cmp::{min, max};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;


macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug)]
struct Cache {
    results: HashMap<(usize, usize, usize), i64>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            results: HashMap::new(),
        }
    }

    fn cached(&self, chars: &[char], to_fit: &[usize], prefix_hashes: usize) -> Option<i64> {
        self.results.get(&(chars.len(), to_fit.len(), prefix_hashes)).map(|i| *i)
    }

    fn insert(&mut self, val: i64, chars: &[char], to_fit: &[usize], prefix_hashes: usize) {
        self.results.insert((chars.len(), to_fit.len(), prefix_hashes), val);
    }
}

#[derive(Debug)]
struct Solution {
    dot_cache: Cache,
    hash_cache: Cache,
}

impl Solution {
    fn new() -> Solution {
        Solution {
            dot_cache: Cache::new(),
            hash_cache: Cache::new(),
        }
    }

    fn handle_dot(&mut self, chars: &[char], to_fit: &[usize], prefix_hashes: usize) -> i64 {
        if prefix_hashes > 0 {
            return 0;
        }
        self.arrangements(&chars[1..], to_fit, 0)
    }

    fn handle_hash(&mut self, chars: &[char], to_fit: &[usize], prefix_hashes: usize) -> i64 {
        let hashes = prefix_hashes + 1;
        if hashes > to_fit[0] {
            panic!("we should catch that earlier");
        }
        if hashes == to_fit[0] {
            if 1 == chars.len() {
                if to_fit.len() == 1 {
                    return 1;
                } else {
                    return 0;
                }
            }
            if chars[1] == '#' {
                return 0;
            }
            return self.arrangements(&chars[2..], &to_fit[1..], 0);
        }
        self.arrangements(&chars[1..], to_fit, hashes)
    }

    fn handle_dot_cached(&mut self, chars: &[char], to_fit: &[usize], prefix_hashes: usize) -> i64 {
        if let Some(v) = self.dot_cache.cached(chars, to_fit, prefix_hashes) {
            v
        } else {
            let v = self.handle_dot(chars, to_fit, prefix_hashes);
            self.dot_cache.insert(v, chars, to_fit, prefix_hashes);
            v
        }
    }

    fn handle_hash_cached(&mut self, chars: &[char], to_fit: &[usize], prefix_hashes: usize) -> i64 {
        if let Some(v) = self.hash_cache.cached(chars, to_fit, prefix_hashes) {
            v
        } else {
            let v = self.handle_hash(chars, to_fit, prefix_hashes);
            self.hash_cache.insert(v, chars, to_fit, prefix_hashes);
            v
        }
    }

    fn arrangements(&mut self, chars: &[char], to_fit: &[usize], prefix_hashes: usize) -> i64 {
        if to_fit.is_empty() {
            if chars.contains(&'#') {
                return 0;
            }
            return 1;
        }

        if chars.is_empty() {
            return 0;
        }

        match chars[0] {
            '.'=> self.handle_dot_cached(chars, to_fit, prefix_hashes),
            '#'=> self.handle_hash_cached(chars, to_fit, prefix_hashes),
            '?' => {
                let mut sol = 0;
                // it is dot
                sol += self.handle_dot_cached(chars, to_fit, prefix_hashes);
                // it is #
                sol += self.handle_hash_cached(chars, to_fit, prefix_hashes);

                sol
            },
            c @ _ => panic!("unexpected char: {}", c)
        }
    }
}

fn multifold<I>(base: &[I], separator: Option<I>, times: usize) -> Vec<I>
where I: Clone
{
    let mut result = Vec::from(base);
    for _ in 0..(times - 1) {
        if let Some(s) = &separator {
            result.push(s.clone());
        }
        result.extend_from_slice(base);
    }
    result
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut solution: i64 = 0;

    for l in BufReader::new(input).lines().map(|l| l.unwrap()) {
        let line = l.trim();

        let mut chunks = line.split_whitespace();

        let chars: Vec<char> = chunks.next().unwrap().chars().collect();
        let knowns: Vec<usize> =
            chunks.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();

        let big_chars = multifold(&chars, Some('?'), 5);
        let big_knowns = multifold(&knowns, None, 5);
        
        dprintln!("line: {:?}", line);
        dprintln!("big chars: {:?}", big_chars);
        dprintln!("big knowns: {:?}", big_knowns);
        let mut sol = Solution::new();
        let arrangements = sol.arrangements(&big_chars, &big_knowns, 0);
        dprintln!("arrgs: {:?}", arrangements);
        solution += arrangements;
    }

    writeln!(output, "{}", solution).unwrap();
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
    fn sample_p1() {
        test_ignore_whitespaces(
            "???.### 1,1,3",
            "1",
        );
    }

    #[test]
    fn sample_p2() {
        test_ignore_whitespaces(
            ".??..??...?##. 1,1,3",
            "16384",
        );
    }

    #[test]
    fn sample_p3() {
        test_ignore_whitespaces(
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "1",
        );
    }

    #[test]
    fn sample_p4() {
        test_ignore_whitespaces(
            "????.#...#... 4,1,1",
            "16",
        );
    }

    #[test]
    fn sample_p5() {
        test_ignore_whitespaces(
            "????.######..#####. 1,6,5",
            "2500",
        );
    }

    #[test]
    fn sample_p6() {
        test_ignore_whitespaces(
            "?###???????? 3,2,1",
            "506250",
        );
    }

    #[test]
    fn sample() {
        test_ignore_whitespaces(
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1",
            "525152",
        );
    }

    #[test]
    fn fully_filled() {
        test_ignore_whitespaces(
            "#.#.### 1,1,3
            .#...#....###. 1,1,3
            .#.###.#.###### 1,3,1,6
            ####.#...#... 4,1,1
            #....######..#####. 1,6,5
            .###.##....# 3,2,1",
            "6",
        );
    }
}
