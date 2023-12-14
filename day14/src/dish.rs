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
struct Rock {
//    x: usize,
    y: usize,
}

impl Rock {
    fn new(_: usize, y: usize) -> Rock { Rock { y } }
}

#[derive(Debug)]
struct Dish {
    rocks: Vec<Rock>,
    height: usize,
}

impl Dish {
    fn from_input<I>(lines: I) -> Dish
        where I: Iterator<Item = String> 
    {
        let mut rocks = Vec::new();
        let mut height = 0;

        let mut first_free = Vec::new();

        for l in lines {
            let line = l.trim();

            if first_free.is_empty() {
                first_free = vec![0; line.len()];
            }
            for (i, c) in line.char_indices() {
                match c {
                    'O' => {
                        rocks.push(Rock::new(i, first_free[i]));
                        first_free[i] += 1;
                    },
                    '#' => {
                        first_free[i] = height + 1;
                    },
                    '.' => {},
                    _ => panic!("Wrong char"),
                }
            }
            height += 1;
        }

        Dish {
            rocks,
            height,
        }
    }

    fn load(&self) -> i64 {
        let mut load = 0;
        for rock in &self.rocks {
            load += self.height as i64 - rock.y as i64;
        }
        load
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let dish = Dish::from_input(lines);
    dprintln!("Dish: {:?}", dish);

    writeln!(output, "{}", dish.load()).unwrap();
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
            "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....",
            "64",
        );
    }

    #[test]
    fn sample_v2() {
        test_ignore_whitespaces(
            "OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....",
            "64",
        );
    }
}
