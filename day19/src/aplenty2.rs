use std::cmp::{min, max};
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
struct Range(i64, i64);


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Range4D {
    dims: Vec<Range>,
}

impl Range4D {
    fn full() -> Range4D {
        Range4D {
            dims: vec![Range(1, 4001); 4], 
        }
    }


    fn intersect(&self, other: &Range4D) -> Range4D {
        let mut dims = Vec::new();

        for i in 0..4 {
            dims.push(
                Range(max(self.dims[i].0, other.dims[i].0),
                    min(self.dims[i].1, other.dims[i].1)));
        }

        Range4D {
            dims,
        }
    }

    fn is_empty(&self) -> bool { self.dims.iter().any(|r| r.0 >= r.1) }

    fn volume(&self) -> i64 {
        self.dims.iter().map(|r| r.1 - r.0).fold(1, |a, b| a * b)
    }
}

fn intersect(many: &[Range4D], other: &Range4D) -> Vec<Range4D>{
    let mut new_ranges = Vec::new();

    for range in many {
        let n = range.intersect(other);
        if !n.is_empty() {
            new_ranges.push(n);
        }
    }

    new_ranges
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
    condition: Range4D,
    anti_condition: Range4D,

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

        let mut condition = Range4D::full();
        let mut anti_condition = Range4D::full();

        if let Some(c) = caps.name("cat") {
            let cat = c.as_str();
            let kind = caps.name("kind").unwrap().as_str();
            let val: i64 = caps.name("val").unwrap().as_str().parse().unwrap();

            let cat_idx = category(cat);
            match kind {
                ">" => {
                    condition.dims[cat_idx] = Range(val + 1, 4001);
                    anti_condition.dims[cat_idx] = Range(1, val + 1);
                },
                "<" => {
                    condition.dims[cat_idx] = Range(1, val);
                    anti_condition.dims[cat_idx] = Range(val, 4001);
                }
                _ => panic!("Don't know this kind"),
            }
        }
        let label = caps.name("label").unwrap().as_str();

        Rule {
            condition,
            anti_condition,

            matched: Decision::from_label(&label),
        }
    }

    // Everything that matches
    fn matching(&self, range: &Range4D) -> Range4D {
        self.condition.intersect(range)
    }

    // Everything that doesn't match
    fn remaining(&self, range: &Range4D) -> Range4D {
        self.anti_condition.intersect(range)
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

    fn accepted_ranges_for(&self, dec: &Decision, cache: &mut HashMap<String, Vec<Range4D>>) -> Vec<Range4D> {
        let label = match dec {
            Decision::Accepted => return vec![Range4D::full()],
            Decision::Rejected => return Vec::new(),
            Decision::Next(l) => l,
        };
        dprintln!("Ranges for {:?}", label);

        if let Some(rs) = cache.get(label) {
            return rs.clone();
        }
        let workflow = self.workflows.get(label).unwrap();

        let mut remaining = Range4D::full();
        let mut result = Vec::new();

        for rule in &workflow.rules {
            dprintln!("--{}-- rule {:?}", label, rule);
            let matched = rule.matching(&remaining);
            remaining = rule.remaining(&remaining);
            dprintln!("--{}-- matching {:?}", label, matched);
            dprintln!("--{}-- remaining {:?}", label, remaining);

            let all_acc = self.accepted_ranges_for(&rule.matched, cache);
            let mut inter = intersect(&all_acc, &matched);

            result.append(&mut inter);
            dprintln!("--{}-- actually accepting {:?}", label, matched);
        }
        //if !remaining.iter().all(|r| r.is_empty()) {
        //    panic!("Rules should have exhausted the posibilities, but still remaining: {:?}, for label: {}",
        //        remaining, label);
        //}
        cache.insert(label.to_string(), result.clone());
        dprintln!(">>{}<< result{:?}", label, result);
        result
    }

    fn count_combinations(&self) -> i64 {
        let mut cache = HashMap::new();
        let ranges = self.accepted_ranges_for(&Decision::Next("in".to_string()), &mut cache);
        dprintln!("Cache: {:?}", cache);

        ranges.iter().map(|r| r.volume()).sum()
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let workflows = Workflows::from_input(&mut lines);
    dprintln!("Workflows: {:?}", workflows);
    
    writeln!(output, "{}", workflows.count_combinations()).unwrap();
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
            "167409079868000",
        );
    }
}
