use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{fmt, usize};

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub node: usize,
    pub cost: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    position: usize,
    cost: usize,
}

pub type Node = Vec<Edge>;

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        return other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position));
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Graph {
    pub fn fmt_path(path: &Vec<usize>) -> String {
        return path
            .iter()
            .map(|&i| ((b'A' + i as u8) as char).to_string())
            .collect::<Vec<_>>()
            .join(" -> ");
    }

    pub fn from_adjacency_matrix(adjacency_matrix: Vec<Vec<usize>>) -> Self {
        let mut graph = Graph { nodes: vec![] };

        let size = adjacency_matrix.len();

        for i in 0..size {
            let mut node: Node = vec![];

            for j in 0..size {
                if adjacency_matrix[i][j] != 0 {
                    node.push(Edge {
                        node: j,
                        cost: adjacency_matrix[i][j],
                    });
                }
            }

            graph.nodes.push(node);
        }

        return graph;
    }

    fn reconstruct_path(
        possible_path: Vec<Option<usize>>,
        start: usize,
        goal: usize,
    ) -> Option<Vec<usize>> {
        let mut path = Vec::new();
        let mut cur = goal;
        path.push(cur);
        while cur != start {
            cur = possible_path[cur]?;
            path.push(cur);
        }

        path.reverse();

        return Some(path);
    }

    pub fn shortest_path(&self, start: usize, goal: usize) -> Option<(usize, Vec<usize>)> {
        let n = self.nodes.len();
        let mut dist = vec![usize::MAX; n];
        let mut visited = BinaryHeap::new();
        let mut prev: Vec<Option<usize>> = vec![None; n];

        dist[start] = 0; // set the starting node to have 0 distance
        visited.push(State {
            position: start,
            cost: 0,
        });

        while let Some(State { cost, position }) = visited.pop() {
            if position == goal {
                return Some((cost, Graph::reconstruct_path(prev, start, goal).unwrap()));
            }

            if cost > dist[position] {
                continue;
            }

            for edge in &self.nodes[position] {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                if next.cost < dist[next.position] {
                    visited.push(next);
                    dist[next.position] = next.cost;
                    prev[edge.node] = Some(position);
                }
            }
        }
        return None;
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!(
            "Edge {{ node: {}, cost {} }}",
            (b'A' + self.node as u8) as char,
            self.cost
        );
        return write!(f, "{}", out);
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        for (i, node) in self.nodes.iter().enumerate() {
            let label = (b'A' + i as u8) as char;
            out.push_str(&format!("{}: [ ", label));
            for edge in node {
                out.push_str(&format!("{}, ", edge));
            }
            out.push_str("]\n");
        }

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = Graph {
            nodes: vec![
                vec![
                    Edge { node: 1, cost: 6 },
                    Edge { node: 2, cost: 4 },
                    Edge { node: 3, cost: 1 },
                ],
                vec![Edge { node: 0, cost: 6 }, Edge { node: 2, cost: 3 }],
                vec![
                    Edge { node: 0, cost: 4 },
                    Edge { node: 1, cost: 3 },
                    Edge { node: 3, cost: 1 },
                ],
                vec![Edge { node: 0, cost: 1 }, Edge { node: 2, cost: 1 }],
            ],
        };

        assert_eq!(graph.shortest_path(0, 1), Some((5, vec![0, 3, 2, 1])));
    }
}
