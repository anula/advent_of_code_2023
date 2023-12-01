use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::iter::FromIterator;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

fn solve<R: BufRead, W: Write>(mut input: R, mut output: W) {

    let mut solution: i64 = 0;


    for line_res in BufReader::new(input).lines() {
        let line = String::from(line_res.unwrap().trim());

        dprintln!("line: {}", line);
        let mut first = '0';
        for c in line.chars() {
            dprintln!("c: {}", c);
            if c.is_digit(10) {
                first = c;
                break;
            }
        }
        let mut last = '0';
        for c in line.chars().rev() {
            if c.is_digit(10) {
                last = c;
                break;
            }
        }
        dprintln!("first: {}, last: {}", first, last);

        solution += String::from_iter(vec![first, last]).parse::<i64>().unwrap();
    }

    writeln!(output, "{}", solution).unwrap();
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    fn test_exact(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        ::solve(input.as_bytes(), &mut actual_out);
        assert_eq!(String::from_utf8(actual_out).unwrap(), output);
    }

    fn test_ignore_whitespaces(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        ::solve(input.as_bytes(), &mut actual_out);
        let actual_out_str = String::from_utf8(actual_out).unwrap();
        let actual_outs = actual_out_str.split_whitespace().collect::<Vec<&str>>();
        let expected_outs = output.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(actual_outs, expected_outs);
    }

    #[test]
    fn sample() {
        test_ignore_whitespaces(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
            "142",
        );
    }

    #[test]
    fn just_numbers() {
        test_ignore_whitespaces(
            "12
            0",
            "12",
        );
    }

    #[test]
    fn single_digit() {
        test_ignore_whitespaces(
            "1
            a5d",
            "66",
        );
    }
}
