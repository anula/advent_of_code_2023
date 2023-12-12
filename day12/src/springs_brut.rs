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

// assumes strings without '?'
fn is_correct(chars: &[char], to_fit: &[usize]) -> bool {
    let s: String = chars.iter().collect();
    let hash_sizes: Vec<usize> = s.split('.').map(|s| s.len()).filter(|&x| x > 0).collect();
    to_fit == &hash_sizes
}

fn replace_next_with(chars: &[char], c: char) -> Option<Vec<char>> {
    let mut chs: Vec<char> = Vec::from(chars);
    for i in 0..chs.len() {
        if chs[i] == '?' {
            chs[i] = c;
            return Some(chs)
        }
    }
    None
}

fn arrangements(chars: &[char], to_fit: &[usize]) -> i64 {
    let mut sol = 0;
    fn count_for(chars: &[char], to_fit: &[usize], c: char) -> i64 {
        match replace_next_with(chars, c) {
            Some(new_chars) => arrangements(&new_chars, to_fit),
            None => {
                if is_correct(chars, to_fit) {
                    dprintln!("Correct: {:?}", chars);
                    1
                } else {
                    0
                }
            }
        }
    }

    if chars.iter().filter(|&c| *c == '?').collect::<Vec<_>>().len() == 0 {
        return if is_correct(chars, to_fit) {
            dprintln!("Correct: {:?}", chars);
            1
        } else {0}
    }
    sol += count_for(chars, to_fit, '#');
    sol += count_for(chars, to_fit, '.');
    sol
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut solution: i64 = 0;

    for l in BufReader::new(input).lines().map(|l| l.unwrap()) {
        let line = l.trim();

        let mut chunks = line.split_whitespace();

        let chars: Vec<char> = chunks.next().unwrap().chars().collect();
        let knowns: Vec<usize> =
            chunks.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();

        println!("line: {:?}", line);
        let arrangements = arrangements(&chars, &knowns);
        println!("arrgs: {:?}", arrangements);
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
    fn test_is_correct() {
        assert_eq!(is_correct(&['#', '.', '#', '.', '.', '.', '#'], &[1,1,1]), true);
        assert_eq!(is_correct(&['#', '.', '#', '.', '.', '.', '#'], &[1,1,2]), false);
        assert_eq!(is_correct(&['#', '.', '#', '.', '.', '#', '#'], &[1,1,2]), true);
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
            "##?.??.???# 3,1,1",
            "4",
        );
    }
}
