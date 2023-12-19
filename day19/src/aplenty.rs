//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

fn category(s: &str) -> usize {
    match s {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("No such category"),
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Decision {
    Next(String),
    Rejected,
    Accepted,
}

impl Decision {
    fn from_label(label: &str) -> Decision {
        match label {
            "R" => Decision::Rejected,
            "A" => Decision::Accepted,
            x => Decision::Next(String::from(x)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Rule {
    min_conditions: Vec<i64>,
    max_conditions: Vec<i64>,

    matched: Decision,
}

impl Rule {
    fn from_str(s: &str) -> Rule {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new(
                r"((?P<cat>[xmas])(?P<kind>[><])(?P<val>\d+):)?(?P<label>.*)"
            ).unwrap();
        }
        let caps = RULE_RE.captures(s).unwrap();

        let mut min_conditions = vec![i64::MIN; 4];
        let mut max_conditions = vec![i64::MAX; 4];

        if let Some(c) = caps.name("cat") {
            let cat = c.as_str();
            let kind = caps.name("kind").unwrap().as_str();
            let val: i64 = caps.name("val").unwrap().as_str().parse().unwrap();

            let cat_idx = category(cat);
            match kind {
                ">" => min_conditions[cat_idx] = val,
                "<" => max_conditions[cat_idx] = val,
                _ => panic!("Don't know this kind"),
            }
        }
        let label = caps.name("label").unwrap().as_str();

        Rule {
            min_conditions,
            max_conditions,

            matched: Decision::from_label(&label),
        }
    }

    fn matches(&self, part: &Part) -> bool {
        let greater = self.min_conditions.iter().enumerate().
            all(|(i, x)| part.vals[i] > *x);
        let less = self.max_conditions.iter().enumerate().
            all(|(i, x)| part.vals[i] < *x);
        greater && less
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Workflow {
    label: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn from_line(line: &str) -> Workflow {
        lazy_static! {
            static ref WORKFLOW_RE: Regex = Regex::new(
                r"(?P<label>.*)\{(?P<rules>.*)\}"
            ).unwrap();
        }
        let caps = WORKFLOW_RE.captures(line).unwrap();

        let mut rules = Vec::new();

        for r in caps.name("rules").unwrap().as_str().split(',') {
            rules.push(Rule::from_str(r));
        }

        Workflow {
            label: caps.name("label").unwrap().as_str().to_string(),
            rules,
        }
    }

    fn decide(&self, part: &Part) -> Decision {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule.matched.clone();
            }
        }
        panic!("No rule matches! Workflow: {:?}, part: {:?}", self, part);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflows {
    workflows: HashMap<String, Workflow>,
}

impl Workflows {
    fn from_input<I>(lines: &mut I) -> Workflows
        where I: Iterator<Item = String>
    {
        let mut workflows = HashMap::new();
        
        while let Some(l) = lines.next() {
            let line = l.trim();
            if line == "" {
                break;
            }
            let w = Workflow::from_line(line);
            workflows.insert(w.label.to_string(), w);
        }

        Workflows {
            workflows,
        }
    }

    fn decide(&self, part: &Part) -> Decision {
        let mut next = Decision::Next("in".to_string());

        dprintln!("Processing part: {:?}", part);
        while let Decision::Next(label) = next {
            dprintln!(" - label: {:?}", label);
            let workflow = self.workflows.get(&label).unwrap();
            next = workflow.decide(part);
        }
        dprintln!(" - decision: {:?}", next);
        next
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Part {
    vals: Vec<i64>,
}

impl Part {
    fn from_line(line: &str) -> Part {
        lazy_static! {
            static ref PART_RE: Regex = Regex::new(
                r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}"
            ).unwrap();
        }
        let caps = PART_RE.captures(line).unwrap();

        let mut vals = vec![0; 4];
        vals[0] = caps.name("x").unwrap().as_str().parse().unwrap();
        vals[1] = caps.name("m").unwrap().as_str().parse().unwrap();
        vals[2] = caps.name("a").unwrap().as_str().parse().unwrap();
        vals[3] = caps.name("s").unwrap().as_str().parse().unwrap();

        Part {
            vals,
        }
    }

    fn sum(&self) -> i64 {
        self.vals.iter().sum()
    }
}



fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let workflows = Workflows::from_input(&mut lines);
    dprintln!("Workflows: {:?}", workflows);
    
    let mut parts = Vec::new();
    for l in lines {
        parts.push(Part::from_line(l.trim()));
    }
    dprintln!("Parts: {:?}", parts);

    let mut solution: i64 = 0;
    for part in parts {
        if workflows.decide(&part) == Decision::Accepted {
            solution += part.sum();
        }
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
            "px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}",
            "19114",
        );
    }
}
