//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::mem::swap;

#[allow(unused_macros)]
macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Node {
    name: String,
    adjs: Vec<String>,
}

impl Node {
    fn parse(line: &str) -> Node {
        let first_parts: Vec<_> = line.split(": ").collect();
        let name = first_parts[0].to_string();
        let mut adjs = Vec::new();
        for a in first_parts[1].split_whitespace() {
            adjs.push(a.trim().to_string());
        }
        Node {
            name,
            adjs,
        }
    }

    fn empty(name: &str) -> Node {
        Node {
            name: name.to_string(),
            adjs: Vec::new(),
        }
    }

    fn as_digraph(&self) -> String {
        let mut st = format!("{} -> {{", self.name);
        for o in &self.adjs {
            st += &format!("{} ", o);
        }
        st += "}";
        st
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn parse<I>(lines: I) -> Graph
        where I: Iterator<Item=String>
    {
        let mut nodes_list = Vec::new();

        for l in lines {
            let line = l.trim();
            nodes_list.push(Node::parse(line));
        }

        let mut nodes: HashMap<_, _> = nodes_list.iter().map(|n| (n.name.clone(), n.clone())).collect();

        for node in nodes_list {
            for a in &node.adjs {
                nodes.entry(a.to_string()).or_insert(Node::empty(a)).adjs.push(node.name.clone());
            }
        }

        Graph {
            nodes,
        }
    }

    fn remove_directed_edge(&mut self, start: &str, end: &str) {
        let start_adjs = &mut self.nodes.get_mut(start).unwrap().adjs;
        let start_idx = start_adjs.iter().position(|el| el == end).unwrap();
        start_adjs.remove(start_idx);
    }

    fn remove_edge(&mut self, node1: &str, node2: &str) {
        self.remove_directed_edge(node1, node2);
        self.remove_directed_edge(node2, node1);
    }

    fn bfs(&self, start: &str, label: i64, labels: &mut HashMap<String, i64>) {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        labels.insert(start.to_string(), label);

        while let Some(node) = queue.pop_front() {

            for a in &self.nodes.get(node).unwrap().adjs {
                if let Some(_) = labels.get(a) {
                    continue;
                }
                labels.insert(a.to_string(), label);
                queue.push_back(a);
            }
        }
    }

    fn multiply_connected_components(&self) -> i64 {
        let mut labels = HashMap::new();
        let mut label = 0;
        for node in self.nodes.keys() {
            if !labels.contains_key(node) {
                self.bfs(node, label, &mut labels);
                label += 1;
            }
        }
        if label != 2 {
            panic!("We were supposed to have 2 components but we have {}", label);
        }
        let num_zeros = labels.iter().filter(|(_, &v)| v == 0).count();
        let num_ones = labels.iter().filter(|(_, &v)| v == 1).count();
        num_zeros as i64 * num_ones as i64
    }

    fn get_all_edges(&self) -> Vec<(String, String)> {
        let mut edges = HashSet::new();

        for node in self.nodes.values() {
            for a in &node.adjs {
                let mut edge = (node.name.clone(), a.clone());
                if edge.0 > edge.1 {
                    swap(&mut edge.0, &mut edge.1);
                }
                edges.insert(edge);
            }
        }
        edges.into_iter().collect()
    }

    #[allow(dead_code)]
    fn as_graph(&self) -> String {

        let mut nodes = String::new();
        let mut edges = String::new();

        for (n, _) in &self.nodes {
            nodes += &format!("{};\n", n);
        }

        for edge in self.get_all_edges() {
            edges += &format!("{} -- {};\n", edge.0, edge.1);
        }

        format!("
            graph G {{
            {}
            {}
            }}
            ", nodes, edges)
		}

    #[allow(dead_code)]
    fn as_digraph(&self) -> String {

        let styling = String::new();
        let mut graph = String::new();

        for (_, no ) in &self.nodes {
            //styling += &mo.digraph_styling();
            //styling += "\n";

            graph += &no.as_digraph();
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
}

fn parse_input<R: BufRead>(input: R) -> Graph {
    let lines = BufReader::new(input).lines().map(|l| l.unwrap());
    Graph::parse(lines)
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut graph = parse_input(input);
    //println!("{}", graph.as_graph());
    //return;

    // Found by graphvizing the graph
    graph.remove_edge("rjs", "mrd");
    graph.remove_edge("gmr", "ntx");
    graph.remove_edge("ncg", "gsk");

    writeln!(output, "{}", graph.multiply_connected_components()).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn test_exact(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        assert_eq!(String::from_utf8(actual_out).unwrap(), output);
    }

    #[allow(dead_code)]
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
        let input =
            "jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr";
        let mut graph = parse_input(input.as_bytes());
        graph.remove_edge("hfx", "pzl");
        graph.remove_edge("bvb", "cmg");
        graph.remove_edge("nvd", "jqt");
        assert_eq!(graph.multiply_connected_components(), 54);
    }
}
