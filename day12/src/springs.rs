//use std::cmp::{min, max};
use std::io::{BufRead, BufReader, Write};


macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

fn arrangements(chars: &[char], to_fit: &[usize], prefix_hashes: usize) -> i64 {
    if to_fit.is_empty() {
        if chars.contains(&'#') {
            return 0;
        }
        return 1;
    }
    if chars.is_empty() {
        return 0;
    }

    fn handle_dot(chars: &[char], to_fit: &[usize], prefix_hashes: usize) -> i64 {
        if prefix_hashes > 0 {
            return 0;
        }
        arrangements(&chars[1..], to_fit, 0)
    }

    fn handle_hash(chars: &[char], to_fit: &[usize], prefix_hashes: usize) -> i64 {
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
            return arrangements(&chars[2..], &to_fit[1..], 0);
        }
        arrangements(&chars[1..], to_fit, hashes)
    }

    match chars[0] {
        '.'=> handle_dot(chars, to_fit, prefix_hashes),

        '#'=> handle_hash(chars, to_fit, prefix_hashes),
        '?' => {
            let mut sol = 0;
            // it is dot
            sol += handle_dot(chars, to_fit, prefix_hashes);
            // it is #
            sol += handle_hash(chars, to_fit, prefix_hashes);

            sol
        },
        c @ _ => panic!("unexpected char: {}", c)
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut solution: i64 = 0;

    for l in BufReader::new(input).lines().map(|l| l.unwrap()) {
        let line = l.trim();

        let mut chunks = line.split_whitespace();

        let chars: Vec<char> = chunks.next().unwrap().chars().collect();
        let knowns: Vec<usize> =
            chunks.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();

        dprintln!("line: {:?}", line);
        let arrangements = arrangements(&chars, &knowns, 0);
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
    fn sample() {
        test_ignore_whitespaces(
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1",
            "21",
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

    #[test]
    fn mine() {
        test_ignore_whitespaces(
            "??#??#?? 1,2,1",
            "2",
        );
    }

    #[test]
    fn mine2() {
        test_ignore_whitespaces(
            "???# 3",
            "1",
        );
    }

    #[test]
    fn mine3() {
        test_ignore_whitespaces(
            "# 3",
            "0",
        );
        test_ignore_whitespaces(
            "? 3",
            "0",
        );
        test_ignore_whitespaces(
            ". 3",
            "0",
        );
    }

    #[test]
    fn mine4() {
        test_ignore_whitespaces(
            "# 1",
            "1",
        );
        test_ignore_whitespaces(
            "? 1",
            "1",
        );
        test_ignore_whitespaces(
            ". 1",
            "0",
        );
    }

    #[test]
    fn mine5() {
        test_ignore_whitespaces(
            "## 1",
            "0",
        );
        test_ignore_whitespaces(
            "## 1,1",
            "0",
        );
    }

    #[test]
    fn from_output() {
        test_ignore_whitespaces(
            "?.#????#??? 1,5",
            "3",
        );
    }

    #[test]
    fn from_output2() {
        test_ignore_whitespaces(
            "#???.#???#?.?.??.? 2,1,5,1,1",
            "5",
        );
    }

    #[test]
    fn from_output_bad() {
        test_ignore_whitespaces(
            "??# 1,1",
            "1",
        );
        test_ignore_whitespaces(
            "??.???# 1,1",
            "4",
        );
        test_ignore_whitespaces(
            "##?.??.???# 3,1,1",
            "4",
        );
    }
}
