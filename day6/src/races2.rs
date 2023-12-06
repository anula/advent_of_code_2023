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
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn default() -> Race {
        Race {
            time: -1,
            distance: -1,
        }
    }

    fn win_possibilities(&self) -> i64 {
        let mut p = 0;

        for i in 1..self.time {
            let hold = i;
            let run = self.time - hold;

            let dist = hold * run;

            if dist > self.distance {
                p += 1;
            }
        }

        p
    }
}

fn concat_to_number<'a, I>(numbers: I) -> i64
        where I: Iterator<Item = &'a str>
{
    let mut total = vec![];
    for num in numbers {
        total.push(num);
    }
    total.concat().parse().unwrap()
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut lines_iter = BufReader::new(input).lines();

    let mut race = Race::default();

    let times_line = lines_iter.next().unwrap().unwrap();
    race.time = concat_to_number(times_line.split_whitespace().skip(1));

    let distance_line = lines_iter.next().unwrap().unwrap();
    race.distance = concat_to_number(distance_line.split_whitespace().skip(1));

    dprintln!("Race: {:?}", race);

    let solution = race.win_possibilities();

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
            "Time:      7  15   30
            Distance:  9  40  200",
            "71503",
        );
    }

    //#[test]
    //fn shorter() {
    //    test_ignore_whitespaces(
    //        "Time:     15   30
    //        Distance: 40  200",
    //        "72",
    //    );
    //}
}
