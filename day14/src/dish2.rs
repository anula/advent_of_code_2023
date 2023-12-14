//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum RockType {
    Cube,
    Rolling,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Rock {
    x: usize,
    y: usize,
    typ: RockType,
}

impl Rock {
    fn new(x: usize, y: usize, typ: RockType) -> Rock { Rock { x, y, typ } }
}

#[derive(Debug)]
struct Dish {
    rocks: Vec<Rock>,

    height: usize,
    width: usize,
}

impl Dish {
    fn from_input<I>(lines: I) -> Dish
        where I: Iterator<Item = String> 
    {
        let mut rocks = Vec::new();
        let mut height = 0;
        let mut width = 0;

        for l in lines {
            let line = l.trim();

            if width == 0 {
                width = line.len();
            }
            for (i, c) in line.char_indices() {
                match c {
                    'O' => {
                        rocks.push(Rock::new(i, height, RockType::Rolling));
                    },
                    '#' => {
                        rocks.push(Rock::new(i, height, RockType::Cube));
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
            width,
        }
    }

    fn slide_north(&mut self) {
        let mut first_free = vec![0; self.width];

        self.rocks.sort_by_key(|r| (r.y, r.x));

        for rock in &mut self.rocks {
            match rock.typ {
                RockType::Rolling => {
                    rock.y = first_free[rock.x];
                    first_free[rock.x] += 1;
                },
                RockType::Cube => {
                    first_free[rock.x] = rock.y + 1;
                },
            }
        }
    }

    fn slide_west(&mut self) {
        let mut first_free = vec![0; self.height];

        self.rocks.sort_by_key(|r| (r.x, r.y));

        for rock in &mut self.rocks {
            match rock.typ {
                RockType::Rolling => {
                    rock.x = first_free[rock.y];
                    first_free[rock.y] += 1;
                },
                RockType::Cube => {
                    first_free[rock.y] = rock.x + 1;
                },
            }
        }
    }

    fn slide_south(&mut self) {
        let mut first_free: Vec<i64> = vec![self.height as i64 - 1; self.width];

        self.rocks.sort_by_key(|r| (-(r.y as i64), r.x));

        for rock in &mut self.rocks {
            match rock.typ {
                RockType::Rolling => {
                    rock.y = first_free[rock.x] as usize;
                    first_free[rock.x] -= 1;
                },
                RockType::Cube => {
                    first_free[rock.x] = rock.y as i64 - 1;
                },
            }
        }
    }

    fn slide_east(&mut self) {
        let mut first_free: Vec<i64> = vec![self.width as i64 - 1; self.height];

        self.rocks.sort_by_key(|r| (-(r.x as i64), r.y));

        for rock in &mut self.rocks {
            match rock.typ {
                RockType::Rolling => {
                    rock.x = first_free[rock.y] as usize;
                    first_free[rock.y] -= 1;
                },
                RockType::Cube => {
                    first_free[rock.y] = rock.x as i64 - 1;
                },
            }
        }
    }

    fn cycle_sliding(&mut self) {
        self.slide_north();
        self.slide_west();
        self.slide_south();
        self.slide_east();
    }

    fn cycle_length(&mut self) -> (usize, usize) {
        let mut already_saw = HashMap::new();

        already_saw.insert(format!("{:?}", self), 0);
        let mut cycles = 1;

        loop {
            self.cycle_sliding();
            let new = format!("{:?}", self);
            if let Some(offset) = already_saw.get(&new) {
                let loop_len = cycles - offset;
                return (*offset, loop_len);
            }
            already_saw.insert(new, cycles);
            cycles += 1;
        }
    }

    fn do_cycling(&mut self, times: usize) {
        let (offset, loop_len) = self.cycle_length();
        for _ in 0..((times - offset) % loop_len) {
            self.cycle_sliding();
        }
    }

    fn load(&self) -> i64 {
        let mut load = 0;
        for rock in &self.rocks {
            load += match rock.typ {
                RockType::Rolling => self.height as i64 - rock.y as i64,
                RockType::Cube => 0,
            };
        }
        load
    }

    fn as_map(&self) -> String {
        let mut result = vec![vec!['.'; self.width]; self.height];
        for rock in &self.rocks {
            result[rock.y][rock.x] = match rock.typ {
                RockType::Rolling => 'O',
                RockType::Cube => '#',
            }
        }
        result.iter().map(|l| l.iter().collect()).collect::<Vec<String>>().join("\n")
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut dish = Dish::from_input(lines);

    dish.do_cycling(1000000000);

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
