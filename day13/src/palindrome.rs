//use std::cmp::{max, min};
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
struct Pattern {
    columns: Vec<String>,
    rows: Vec<String>,
}

impl Pattern {
    fn from_lines<I>(lines: &mut I) -> Option<Pattern>
        where I: Iterator<Item = String>
    {
        let mut columns = Vec::<Vec<char>>::new();
        let mut rows = Vec::<Vec<char>>::new();

        let mut row = 0;
        while let Some(l) = lines.next() {
            let line = l.trim();
            if line == "" {
                break
            }
            rows.push(Vec::new());

            for (i, c) in line.char_indices() {
                if columns.len() <= i {
                    columns.push(Vec::new());
                }
                columns[i].push(c);
                rows[row].push(c);
            }

            row += 1;
        }
        if row == 0 {
            return None;
        }

        Some(Pattern {
            columns: columns.into_iter().map(|chs| chs.into_iter().collect::<String>()).collect(),
            rows: rows.into_iter().map(|chs| chs.into_iter().collect::<String>()).collect(),
        })
    }

    fn find_palindrome(arr: &[String]) -> Option<usize> {
        dprintln!("find_palindrome: {:?}", arr);
        for i in 0..(arr.len() - 1) {
            dprintln!("Try i: {:?}", i);
            if (arr.len() - i) % 2 != 0 { continue; }
            dprintln!("Going on..");

            let mut left_i = i;
            let mut right_i = arr.len() - 1;

            while left_i < right_i && arr[left_i] == arr[right_i] {
                left_i += 1;
                right_i -= 1;
            }
            dprintln!("Final left_i: {:?}, right_i: {:?}", left_i, right_i);
            dprintln!("arr[left_i]: {:?}, arr[right_i]: {:?}", arr[left_i], arr[right_i]);
            if left_i == right_i + 1 && arr[left_i] == arr[right_i] {
                dprintln!("Thus returning: {:?}", left_i);
                return Some(left_i);
            }
        }
        None
    }

    fn find_rev_palindrome(arr: &[String]) -> Option<usize> {
        let arr_rev = arr.iter().rev().map(|x| x.clone()).collect::<Vec<String>>();
        match Self::find_palindrome(&arr_rev) {
            Some(p) => Some(arr.len() - p),
            None => None,
        }
    }

    fn find_longest_palindrome(arr: &[String]) -> Option<usize> {
        let mut found = None;
        if let Some(c) = Self::find_palindrome(arr) {
            found = Some(c);
        }
        if let Some(c) = Self::find_rev_palindrome(arr) {
            if let Some(pc) = found {
                let prev_len = arr.len() - pc;
                let curr_len = c;
                if curr_len > prev_len {
                    found = Some(c);
                }
            } else {
                found = Some(c);
            }
        }
        found
    }

    fn summarize(&self) -> i64 {
        if let Some(c) = Self::find_longest_palindrome(&self.columns) {
            return c as i64;
        }

        if let Some(c) = Self::find_longest_palindrome(&self.rows) {
            return c as i64 * 100;
        }

        println!("Not found for: {:?}", self);
        panic!("No Palindrome found!");
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut solution: i64 = 0;


    let mut lines = BufReader::new(input).lines().map(|l| l.unwrap());
    while let Some(pat) = Pattern::from_lines(&mut lines) {
        dprintln!("Pattern: {:?}", pat);
        solution += pat.summarize();
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
    fn sample() {
        test_ignore_whitespaces(
            "#.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#",
            "405",
        );
    }

    #[test]
    fn mine() {
        test_ignore_whitespaces(
            "#.#....##",
            "8",
        );
    }

    #[test]
    fn mine2() {
        test_ignore_whitespaces(
            "######.##",
            "3",
        );
    }

    #[test]
    fn mine3() {
        test_ignore_whitespaces(
            "####.#.#",
            "2",
        );
    }

    #[test]
    fn input() {
        test_ignore_whitespaces(
            ".#.####
            ##..#.#
            ##..#.#
            .#.####
            ..#..#.
            ####.#.
            #.#.#.#
            .#..#.#
            ##.##..
            #.#..#.",
            "200",
        );
    }
}
