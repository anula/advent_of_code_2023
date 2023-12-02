//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use regex::Regex;
use lazy_static::lazy_static;
//use std::fmt;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug)]
struct Set {
    red: i64,
    green: i64,
    blue: i64,
}

impl Set {
    fn from_string(st: &str) -> Set {

        let mut set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        for color in st.trim().split(",").collect::<Vec<_>>() {
            let c = color.trim();


            let split_space = c.split(" ").collect::<Vec<_>>();

            let num = split_space[0].parse::<i64>().unwrap();
            if c.ends_with("red") {
                set.red += num;
            } else if c.ends_with("green") {
                set.green += num;
            } else if c.ends_with("blue") {
                set.blue += num;
            }
        }

        return set;
    }
}

#[derive(Debug)]
struct Game {
    number: i64,
    sets: Vec<Set>,
}

impl Game {
    fn from_string(st: &str) -> Game {
        let s = st.trim();

        lazy_static! {
            static ref GAME_RE : Regex = Regex::new(
                r"Game (?P<game>\d+): (?P<sets>.*)"
            ).unwrap();
        }

        let caps = GAME_RE.captures(s).unwrap();
        let mut sets = Vec::<Set>::new();

        for set in caps.name("sets").unwrap().as_str().split(";").collect::<Vec<_>>() {
            sets.push(Set::from_string(set));
        }

        Game {
            number: caps.name("game").unwrap().as_str().parse::<i64>().unwrap(),
            sets: sets,
        }
    }

    fn is_possible(&self, total_red: i64, total_green: i64, total_blue: i64) -> bool {
        for set in &self.sets {
            if set.red > total_red || set.green > total_green || set.blue > total_blue {
                return false;
            }
        }
        return true;
    }

    fn power(&self) -> i64 {
        let max_red = self.sets.iter().map(|s| s.red).max().unwrap();
        let max_green = self.sets.iter().map(|s| s.green).max().unwrap();
        let max_blue = self.sets.iter().map(|s| s.blue).max().unwrap();

        return max_red * max_green * max_blue;
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut solution: i64 = 0;

    for line_res in BufReader::new(input).lines() {
        let line = line_res.unwrap();
        let game = Game::from_string(&line);
        dprintln!("Game: {:?}", game);

        solution += game.power();
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
    use crate::cubes2::solve;

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
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            "2286",
        );
    }

    #[test]
    fn one_line() {
        test_exact(
            "Game 1: 12 red, 13 green, 14 blue",
            "2184\n",
        );
    }
}
