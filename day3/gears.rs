use std::cmp::{max, min};
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
enum SchematicPart {
    Dot,
    Symbol,
    Num(usize),
}

#[derive(Debug)]
struct Schematic {
    nums: Vec<i64>,
    content: Vec<Vec<SchematicPart>>,
    symbols: Vec<(usize, usize)>,
}

impl Schematic {
    fn new_empty() -> Schematic {
        Schematic {
            nums: Vec::new(),
            content: Vec::new(),
            symbols: Vec::new(),
        }
    }

    fn add_line(&mut self, line: &str) {
        let row = self.content.len();
        self.content.push(Vec::new());
        let mut char_it = line.char_indices().peekable();
        while let Some((i, c)) = char_it.next() {
            match c {
                c if c.is_digit(10) => {
                    let num_start = i;
                    let mut num_end = i + 1;
                    self.content[row].push(SchematicPart::Num(self.nums.len()));
                    while let Some((_, nc)) = char_it.peek() {
                        if !nc.is_digit(10) { break; }
                        let Some((j, _d)) = char_it.next() else { panic!() };
                        num_end = j + 1;
                        self.content[row].push(SchematicPart::Num(self.nums.len()));
                    }
                    self.nums.push(line[num_start..num_end].parse::<i64>().unwrap());
                }
                '.' => self.content[row].push(SchematicPart::Dot),
                _ => {
                    self.symbols.push((row, self.content[row].len()));
                    self.content[row].push(SchematicPart::Symbol);
                },
            }
        }
    }

    fn sum_of_parts(&self) -> i64 {
        let mut parts = Vec::<usize>::new();
        for (row, col) in &self.symbols {
            let min_row = max(0, (*row as i32) - 1) as usize;
            let max_row = min(self.content.len(), *row + 2);
            let min_col = max(0, (*col as i32) - 1) as usize;
            let max_col = min(self.content[0].len(), *col + 2);

            for i in min_row..max_row {
                for j in min_col..max_col {
                    if let SchematicPart::Num(idx) = self.content[i][j] {
                        parts.push(idx);
                    }
                }
            }
        }
        parts.sort();
        parts.dedup();

        parts.into_iter().map(|idx| self.nums[idx]).sum()
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut schema = Schematic::new_empty();

    for line_res in BufReader::new(input).lines() {
        let line = line_res.unwrap();

        schema.add_line(line.trim());
    }
    dprintln!("{:?}", schema);

    writeln!(output, "{}", schema.sum_of_parts()).unwrap();
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
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..",
            "4361",
        );
    }

    #[test]
    fn test_single_line() {
        test_exact(
            "12*10...3..10#",
            "32\n",
        );
    }
}
