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
        //dprintln!("find_palindrome: {:?}", arr);
        for i in 0..(arr.len() - 1) {
            //dprintln!("Try i: {:?}", i);
            if (arr.len() - i) % 2 != 0 { continue; }
            //dprintln!("Going on..");

            let mut left_i = i;
            let mut right_i = arr.len() - 1;

            while left_i < right_i && arr[left_i] == arr[right_i] {
                left_i += 1;
                right_i -= 1;
            }
            //dprintln!("Final left_i: {:?}, right_i: {:?}", left_i, right_i);
            //dprintln!("arr[left_i]: {:?}, arr[right_i]: {:?}", arr[left_i], arr[right_i]);
            if left_i == right_i + 1 && arr[left_i] == arr[right_i] {
                //dprintln!("Thus returning: {:?}", left_i);
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

    fn find_longest_palindrome(arr: &[String], different: i64) -> Option<usize> {
        let mut found = None;
        if let Some(c) = Self::find_palindrome(arr) {
            if c as i64 != different {
                found = Some(c);
            }
        }
        if let Some(c) = Self::find_rev_palindrome(arr) {
            if c as i64 != different {
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
        }
        found
    }

    fn summarize(&self, different: i64) -> Option<i64> {
        if let Some(c) = Self::find_longest_palindrome(&self.columns, different) {
            if c as i64 != different {
                return Some(c as i64);
            }
        }

        let diff = if different % 100 == 0 {
            different / 100
        } else {
            -1
        };
        if let Some(c) = Self::find_longest_palindrome(&self.rows, diff) {
            if c as i64 != diff {
                return Some(c as i64 * 100);
            }
        }

        None
    }

    fn modified_summary(&mut self) -> i64 {
        let original = self.summarize(-1).unwrap();
        dprintln!("original: {}", original);
        for col in 0..self.columns.len() {
            for row in 0..self.rows.len() {
                let char_at: char = self.columns[col].chars().nth(row).unwrap();
                let new_char = match char_at {
                    '#' => ".",
                    '.' => "#",
                    _ => panic!("Wrong char"),
                };

                self.columns[col].replace_range(row..(row+1), new_char);
                self.rows[row].replace_range(col..(col+1), new_char);

                if let Some(r) = self.summarize(original) {
                    dprintln!("Found reflection for: {:?}", self);
                    return r;
                }

                let old_char = String::from(char_at);
                self.columns[col].replace_range(row..(row+1), &old_char);
                self.rows[row].replace_range(col..(col+1), &old_char);
            }
        }
        panic!("Did not find solution");
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut solution: i64 = 0;


    let mut lines = BufReader::new(input).lines().map(|l| l.unwrap());
    while let Some(mut pat) = Pattern::from_lines(&mut lines) {
        dprintln!("Pattern: {:?}", pat);
        let res = pat.modified_summary();
        dprintln!("Summarization: {:?}", res);
        solution += res;
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
            "400",
        );
    }

   // #[test]
   // fn input() {
   //     test_ignore_whitespaces(
   //         ".#.####
   //          ##..#.#
   //          ##..#.#
   //          .#.####
   //          ..#..#.
   //          ####.#.
   //          #.#.#.#
   //          .#..#.#
   //          ##.##..
   //          #.#..#.",
   //         "200",
   //     );
   // }
}
