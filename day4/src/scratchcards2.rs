use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;
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

#[derive(Debug)]
struct Scratchcard {
    number: i64,
    winning_numbers: HashSet<i64>,
    chosen_numbers: Vec<i64>,
    matches: usize,
}

impl Scratchcard {
    fn from_string(full_line: &str) -> Scratchcard {
        let line = full_line.trim();

        lazy_static! {
            static ref CARD_RE: Regex = Regex::new(
                r"Card +(?P<num>\d+): (?P<win>[\d ]*)\|(?P<have>[\d ]*)"
            ).unwrap();
        }

        let caps = CARD_RE.captures(line).unwrap();
        //dprintln!("caps: {:?}", caps);

        let mut win = HashSet::<i64>::new();

        for num in caps.name("win").unwrap().as_str().split_whitespace().collect::<Vec<_>>() {
            win.insert(num.parse::<i64>().unwrap());
        }

        let mut chosen = Vec::<i64>::new();
        let mut matches = 0;
        for num in caps.name("have").unwrap().as_str().split_whitespace().collect::<Vec<_>>() {
            let inum = num.parse::<i64>().unwrap();
            chosen.push(inum);
            if win.contains(&inum) {
                matches += 1;
            }
        }

        Scratchcard {
            number: caps.name("num").unwrap().as_str().parse::<i64>().unwrap(),
            winning_numbers: win,
            chosen_numbers: chosen,
            matches: matches,
        }
    }
}


fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut solution: i64 = 0;

    let mut cards = Vec::<Scratchcard>::new();

    for line_res in BufReader::new(input).lines() {
        let line = line_res.unwrap();
        let card = Scratchcard::from_string(&line);
        dprintln!("Card: {:?}", card);
        cards.push(card);
    }

    let mut card_nums = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        solution += card_nums[i];
        for j in (i+1)..(min(i+card.matches+1, card_nums.len())) {
            card_nums[j] += card_nums[i];
        }
    }
    dprintln!("nums: {:?}", card_nums);

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
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            "30",
        );
    }

    #[test]
    fn test_one_line() {
        test_exact(
            "Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "1\n",
        );
    }
}
