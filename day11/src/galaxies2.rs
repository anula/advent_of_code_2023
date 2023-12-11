//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::{HashMap};

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct GalaxyMap {
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,

    galaxies: Vec<Galaxy>,
}

impl Galaxy {
    fn from_coords(x: usize, y: usize) -> Galaxy { Galaxy {x, y} }
}

impl GalaxyMap {
    fn from_input<I>(lines: I) -> GalaxyMap
        where I: Iterator<Item = String>
    {

        let mut galaxies_per_col = HashMap::new();
        let mut galaxies = Vec::new();
        let mut empty_rows = Vec::new();
        for (row, l) in lines.enumerate() {
            let line = l.trim();
            let mut galaxies_per_row = 0;

            for (col, c) in line.char_indices() {
                let entry = galaxies_per_col.entry(col).or_insert(0);
                match c {
                    '#' => {
                        galaxies_per_row += 1;
                        *entry += 1;
                        galaxies.push(Galaxy::from_coords(col, row));
                    },
                    _ => { }
                }
            }
            if galaxies_per_row == 0 {
                empty_rows.push(row);
            }
        }
        let mut empty_columns = galaxies_per_col.iter()
            .filter(|(_, &num)| num == 0).map(|(col, _)| *col).collect::<Vec<usize>>();
        empty_columns.sort();
        GalaxyMap {
            empty_rows: empty_rows,
            empty_columns: empty_columns,

            galaxies: galaxies,
        }
    }

    fn expand(&mut self) {
        let mut row_idx = 0;
        for g in &mut self.galaxies {
            while row_idx < self.empty_rows.len() && g.y > self.empty_rows[row_idx] {
                row_idx += 1;
            }
            g.y += row_idx * (1000000 - 1);
        }

        self.galaxies.sort_by_key(|g| g.x);

        let mut col_idx = 0;
        for g in &mut self.galaxies {
            while col_idx < self.empty_columns.len() && g.x > self.empty_columns[col_idx] {
                col_idx += 1;
            }
            g.x += col_idx * (1000000 - 1);
        }
    }

    fn sum_distances(&self) -> i64 {
        let mut sum: i64 = 0;
        for i in 0..self.galaxies.len() {
            for j in (i+1)..self.galaxies.len() {
                let g1 = &self.galaxies[i];
                let g2 = &self.galaxies[j];
                sum += ((g1.x as i64) - (g2.x as i64)).abs();
                sum += ((g1.y as i64) - (g2.y as i64)).abs();
            }
        }
        sum
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines_it = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut galaxy_map = GalaxyMap::from_input(lines_it);
    dprintln!("Map: {:?}", galaxy_map);
    galaxy_map.expand();
    dprintln!("Expanded: {:?}", galaxy_map);

    writeln!(output, "{}", galaxy_map.sum_distances()).unwrap();
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
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
            "82000210",
        );
    }
}
