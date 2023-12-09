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
struct History {
    sequences: Vec<Vec<i64>>,
}

impl History {

    fn from_string(line: &str) -> History {
        History {
            sequences: vec![
                line.split_whitespace().map(|x| x.parse().unwrap()).collect(),
            ],
        }
    }

    fn compute_diffs(base: &[i64]) -> Vec<i64> {
        let mut diffs = Vec::with_capacity(base.len()-1);

        for i in 0..(base.len()-1) {
            diffs.push(base[i+1] - base[i])
        }
        diffs
    }

    fn build_diffs_sequence(&mut self) {
        self.sequences.push(History::compute_diffs(&self.sequences[0]));
        fn end_seq(seq: &[i64]) -> bool {
            seq.iter().all(|&x| x == 0)
        }

        while !end_seq(self.sequences.last().unwrap()) {
            self.sequences.push(History::compute_diffs(self.sequences.last().unwrap()));
        }
    }

    fn compute_prev(&self) -> i64 {
        let mut prevs = vec![0 as i64; self.sequences.len()];
        for i in (0..prevs.len()-1).rev() {
            prevs[i] = self.sequences[i][0] - prevs[i+1];
        }
        return prevs[0];
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut solution: i64 = 0;

    for line in BufReader::new(input).lines().map(|l| l.unwrap()) {
        let mut history = History::from_string(&line);
        history.build_diffs_sequence();
        dprintln!("History: {:?}", history);
        solution += history.compute_prev();
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
            "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45",
            "2",
        );
    }
}
