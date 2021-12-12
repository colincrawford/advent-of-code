use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Cave {
    is_large: bool,
    name: String,
}

impl Cave {
    fn is_small(&self) -> bool {
        !self.is_large
    }
}

struct Graph {
    // name of cave -> connected caves
    edges: HashMap<String, Vec<Cave>>,
}

impl Graph {
    fn insert_edge(&mut self, vert1: Cave, vert2: Cave) {
        let v1 = vert1.clone();
        let v2 = vert2.clone();
        if !self.edges.contains_key(&vert1.name) {
            self.edges.insert(vert1.name, vec![v2]);
        } else {
            self.edges
                .get_mut(&vert1.name)
                .expect("No edge found")
                .push(v2);
        }

        if !self.edges.contains_key(&vert2.name) {
            self.edges.insert(vert2.name, vec![v1]);
        } else {
            self.edges
                .get_mut(&vert2.name)
                .expect("No edge found")
                .push(v1);
        }
    }
}

fn name_to_cave(name: &str) -> Cave {
    let is_large = name == "start" || name == "end" || name.chars().all(|c| c.is_ascii_uppercase());
    Cave {
        is_large,
        name: String::from(name),
    }
}

fn parse_input(input: &str) -> Graph {
    let mut graph = Graph {
        edges: HashMap::new(),
    };
    let edges = input.lines().map(|line| {
        let mut path = line.split("-");
        let v1 = name_to_cave(path.next().expect("Invalid input line"));
        let v2 = name_to_cave(path.next().expect("Invalid input line"));
        (v1, v2)
    });
    for (v1, v2) in edges {
        graph.insert_edge(v1, v2);
    }
    graph
}

fn traverse(
    graph: &Graph,
    current_vertex: &str,
    path: &mut HashSet<String>,
    has_visited_small_cave_twice: bool,
) -> u32 {
    if current_vertex == "end" {
        return 1;
    }

    let mut paths = 0;
    let edges = graph.edges.get(current_vertex).expect("No edges");
    for edge in edges {
        if edge.name == "start" {
            continue;
        }

        let had_before = path.contains(&edge.name);
        path.insert(edge.name.clone());

        if edge.is_small() && had_before {
            if has_visited_small_cave_twice {
                continue;
            }
            paths += traverse(graph, &edge.name, path, true);
        } else {
            paths += traverse(graph, &edge.name, path, has_visited_small_cave_twice);
        }

        if !had_before {
            path.remove(&edge.name);
        }
    }
    paths
}

pub fn day12_part1(input: &str) -> String {
    let graph = parse_input(input);
    format!("{:?}", traverse(&graph, "start", &mut HashSet::new(), true))
}

pub fn day12_part2(input: &str) -> String {
    let graph = parse_input(input);
    format!("{:?}", traverse(&graph, "start", &mut HashSet::new(), false))
}
