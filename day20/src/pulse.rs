//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

#[allow(unused_macros)]
macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct FlipFlopState {
    on: bool,
}

impl FlipFlopState {
    fn new() -> FlipFlopState {
        FlipFlopState {
            on: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ConjuctionState {
    inputs: HashMap<String, bool>,
}

impl ConjuctionState {
    fn new() -> ConjuctionState {
        ConjuctionState {
            inputs: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop(FlipFlopState),
    Conjuction(ConjuctionState),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module {
    name: String,
    state: ModuleType,

    outputs: Vec<String>,
}

impl Module {
    fn from_line(line: &str) -> Module {
        lazy_static! {
            static ref MODULE_RE: Regex = Regex::new(
                r"(?P<special>[%&])?(?P<name>\S+) -> (?P<outs>.+)"
            ).unwrap();
        }
        let caps = MODULE_RE.captures(line).unwrap();

        let special = caps.name("special");
        let name = caps.name("name").unwrap().as_str().to_string();
        let outs = caps.name("outs").unwrap().as_str();

        let state = if let Some(typ) = special {
            match typ.as_str() {
                "%" => ModuleType::FlipFlop(FlipFlopState::new()),
                "&" => ModuleType::Conjuction(ConjuctionState::new()),
                _ => panic!("Wrong special: {:?}", typ),
            }
        } else {
            ModuleType::Broadcast
        };


        Module {
            name,
            state,

            outputs: outs.split(", ").map(|s| s.to_string()).collect(),
        }
    }

    fn update_state_and_out(&mut self, sender: &str, in_pulse: bool) -> Option<bool> {
        match &mut self.state {
            ModuleType::Broadcast => { Some(in_pulse) },
            ModuleType::FlipFlop(st) => {
                if !in_pulse {
                    st.on = !st.on;
                    Some(st.on)
                } else {
                    None
                }
            },
            ModuleType::Conjuction(st) => {
                *st.inputs.get_mut(sender).unwrap() = in_pulse;
                Some(!st.inputs.values().all(|v| *v))
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Graph {
    modules: HashMap<String, Module>,
}

impl Graph {
    fn build<I>(lines: I) -> Graph
        where I: Iterator<Item = String>
    {
        let mut modules = HashMap::new();
        let mut cons = HashMap::new();

        for l in lines {
            let line = l.trim();
            let module = Module::from_line(line);

            if let ModuleType::Conjuction(_) = module.state {
                cons.insert(module.name.clone(), vec![]);
            }

            modules.insert(module.name.clone(), module);
        }

        for (_, module) in &modules {
            for out in &module.outputs {
                if let Some(ins) = cons.get_mut(out) {
                    ins.push(module.name.clone());
                }
            }
        }

        for (name, ins) in cons {
            if let ModuleType::Conjuction(con_st) = &mut modules.get_mut(&name).unwrap().state {
                for in_m in ins {
                    con_st.inputs.insert(in_m, false);
                }
            } else {
                panic!("This really should be conjuction");
            }
        }

        Graph {
            modules,
        }
    }

    fn bfs(&mut self, start: &str) -> (i64, i64) {
        let mut high_pulses = 0;
        let mut low_pulses = 1;

        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), false, start.to_string()));

        while let Some((sender, pulse, node)) = queue.pop_front() {
            dprintln!("From queue: {}, {}, {}", sender, pulse, node);
            let mo = if let Some(m) = self.modules.get_mut(&node) {
                m
            } else {
                // Just output vert;
                continue;
            };
            let maybe_out = mo.update_state_and_out(&sender, pulse);
            dprintln!("-- maybe_out: {:?}", maybe_out);

            if let Some(out) = maybe_out {
                for n in &mo.outputs {
                    dprintln!("-- pushing: {:?}", (node.clone(), out, n.to_string()));
                    queue.push_back((node.clone(), out, n.to_string()));

                    if out {
                        high_pulses += 1;
                    } else {
                        low_pulses += 1;
                    }
                }
            }
        }

        (low_pulses, high_pulses)
    }

    fn count_pulses_after(&mut self, times: i64) -> i64 {
        let mut low_sums = vec![0];
        let mut high_sums = vec![0];

        let mut past_states = HashMap::new();

        let mut step = 1;
        let offset;
        let cycle_len;

        loop {
            let (lows, highs) = self.bfs("broadcaster");
            low_sums.push(low_sums.last().unwrap() + lows);
            high_sums.push(high_sums.last().unwrap() + highs);

            if step == times {
                offset = times;
                cycle_len = 0;
                break;
            }

            let st = format!("{:?}", self);
            if let Some(off) = past_states.get(&st) {
                offset = *off - 1;
                cycle_len = step - *off;
                break;
            }
            past_states.insert(st, step);
            step += 1;
        }

        dprintln!("Offset: {}, cycle_len: {}", offset, cycle_len);
        dprintln!("low_sums: {:?}", low_sums);
        dprintln!("high_sums: {:?}", high_sums);

        let start_cycle = (offset) as usize;
        let end_cycle = (offset + cycle_len) as usize;
        let cyc_count = if cycle_len > 0 { (times - offset) / cycle_len } else { 0 };
        let cyc_reminder = if cycle_len > 0 { ((times - offset) % cycle_len) as usize } else { 0 };

        let lows = low_sums[offset as usize] +
            (low_sums[end_cycle] - low_sums[start_cycle]) * cyc_count +
            low_sums[start_cycle + cyc_reminder] - low_sums[start_cycle];
        let highs = high_sums[offset as usize] +
            (high_sums[end_cycle] - high_sums[start_cycle]) * cyc_count +
            high_sums[start_cycle + cyc_reminder] - high_sums[start_cycle];

        lows * highs
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut graph = Graph::build(lines);
    dprintln!("Graph: {:?}", graph);

    writeln!(output, "{:?}", graph.count_pulses_after(1000)).unwrap();
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
    fn sample1() {
        test_ignore_whitespaces(
            "broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a",
            "32000000",
        );
    }

    #[test]
    fn sample2() {
        test_ignore_whitespaces(
            "broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output",
            "11687500",
        );
    }
}
