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
    inputs: Vec<String>,
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
            inputs: Vec::new(),
        }
    }

    fn as_digraph(&self) -> String {
        let mut st = format!("{} -> {{", self.name);
        for o in &self.outputs {
            st += &format!("{} ", o);
        }
        st += "}";
        st
    }

    fn digraph_styling(&self) -> String {
        let shape = match self.state {
            ModuleType::Broadcast => "doublecircle",
            ModuleType::FlipFlop(_) => "triangle",
            ModuleType::Conjuction(_) => "polygon",
        };

        format!("{} [shape={}]", self.name, shape)
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

    fn bfs(&mut self, start: &str, end: &str) -> Vec<bool> {
        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), false, start.to_string()));

        let mut end_pulses = Vec::new();

        while let Some((sender, pulse, node)) = queue.pop_front() {
            let mo = if let Some(m) = self.modules.get_mut(&node) {
                m
            } else {
                // Just output vert;
                continue;
            };
            if node == end {
                end_pulses.push(pulse);
                // We pretend end has no outputs.
                continue;
            }
            let maybe_out = mo.update_state_and_out(&sender, pulse);

            if let Some(out) = maybe_out {
                for n in &mo.outputs {
                    queue.push_back((node.clone(), out, n.to_string()));
                }
            }
        }

        end_pulses
    }

    #[allow(dead_code)]
    fn as_digraph(&self) -> String {

        let mut styling = String::new();
        let mut graph = String::new();

        for (_, mo) in &self.modules {
            styling += &mo.digraph_styling();
            styling += "\n";

            graph += &mo.as_digraph();
            graph += "\n";
        }

        format!("
            digraph G {{
            {{
            {}
            }}
            {}
            }}
            ", styling, graph)
    }

    fn find_false(&mut self, start: &str, end: &str) -> i64 {
        let mut steps = 1;
        loop {
            let end_values = self.bfs(start, end);
            if end_values.is_empty() {
                panic!("End values can be != 1, {:?}", end_values);
            }
            if end_values.iter().any(|x| !x) {
                println!("out {:?}", end_values);
                break;
            }
            steps += 1;
        }
        steps
    }
}

fn solve<R: BufRead, W: Write>(input: R, _: W) {
    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    let mut graph = Graph::build(lines);

    // Uncomment to create a dot file to visualize in graphviz
    //println!("{:?}", graph.as_digraph());

    println!("Find first zero (pk, xf): {:?}", graph.find_false("pk", "xf"));
    println!("Find first zero (xt, hn): {:?}", graph.find_false("xt", "hn"));
    println!("Find first zero (vk, fz): {:?}", graph.find_false("vk", "fz"));
    println!("Find first zero (km, mp): {:?}", graph.find_false("km", "mp"));

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
    fn sample2() {
        test_ignore_whitespaces(
            "broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> rx",
            "1",
        );
    }
}
