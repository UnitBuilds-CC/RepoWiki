use std::collections::{HashMap, HashSet, VecDeque};

use repowiki_core::models::{CallEdge, ProjectIndex};

pub fn trace_flow(
    index: &ProjectIndex,
    entry_symbol: &str,
    max_depth: usize,
) -> Vec<CallEdge> {
    let mut result = Vec::new();
    let mut visited: HashSet<(String, String)> = HashSet::new();
    let mut queue: VecDeque<(String, String, usize)> = VecDeque::new();

    let outgoing = build_outgoing(index);

    for edge in &index.call_graph {
        if edge.from_symbol == entry_symbol {
            let key = (edge.from_symbol.clone(), edge.to_symbol.clone());
            if visited.insert(key) {
                result.push(edge.clone());
                queue.push_back((edge.to_file.clone(), edge.to_symbol.clone(), 1));
            }
        }
    }

    while let Some((file, symbol, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }
        let key = (file.clone(), symbol.clone());
        if let Some(edges) = outgoing.get(&key) {
            for edge in edges {
                let visit_key = (edge.from_symbol.clone(), edge.to_symbol.clone());
                if visited.insert(visit_key) {
                    result.push(edge.clone());
                    queue.push_back((edge.to_file.clone(), edge.to_symbol.clone(), depth + 1));
                }
            }
        }
    }

    result
}

fn build_outgoing(
    index: &ProjectIndex,
) -> HashMap<(String, String), Vec<CallEdge>> {
    let mut map: HashMap<(String, String), Vec<CallEdge>> = HashMap::new();
    for edge in &index.call_graph {
        let key = (edge.from_file.clone(), edge.from_symbol.clone());
        map.entry(key).or_default().push(edge.clone());
    }
    map
}
