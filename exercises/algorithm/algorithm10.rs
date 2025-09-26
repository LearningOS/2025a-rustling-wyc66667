/*
    graph
    This problem requires you to implement a basic graph function
*/

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_list: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> Self {
        UndirectedGraph {
            adjacency_list: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_list
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_list
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (u, v, weight) = edge;
        
        // 添加 u 到 v 的边
        self.adjacency_table_mutable()
            .entry(u.to_string())
            .or_insert_with(Vec::new)
            .push((v.to_string(), weight));
        
        // 添加 v 到 u 的边（无向图双向性）
        self.adjacency_table_mutable()
            .entry(v.to_string())
            .or_insert_with(Vec::new)
            .push((u.to_string(), weight));
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        // 如果节点不存在则添加，返回true；否则返回false
        self.adjacency_table_mutable()
            .insert(node.to_string(), Vec::new())
            .is_none()
    }

    fn add_edge(&mut self, edge: (&str, &str, i32));

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().contains_key(node)
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from, neighbors) in self.adjacency_table() {
            for (to, weight) in neighbors {
                edges.push((from, to, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        
        // 测试添加第一条边
        graph.add_edge(("a", "b", 5));
        let edges1 = graph.edges();
        assert!(edges1.contains(&(&"a".to_string(), &"b".to_string(), 5)), "a->b 边未添加");
        assert!(edges1.contains(&(&"b".to_string(), &"a".to_string(), 5)), "b->a 边未添加");
        
        // 添加第二条边
        graph.add_edge(("b", "c", 10));
        let edges2 = graph.edges();
        assert!(edges2.contains(&(&"b".to_string(), &"c".to_string(), 10)), "b->c 边未添加");
        assert!(edges2.contains(&(&"c".to_string(), &"b".to_string(), 10)), "c->b 边未添加");
        
        // 添加第三条边
        graph.add_edge(("c", "a", 7));
        let edges3 = graph.edges();
        assert!(edges3.contains(&(&"c".to_string(), &"a".to_string(), 7)), "c->a 边未添加");
        assert!(edges3.contains(&(&"a".to_string(), &"c".to_string(), 7)), "a->c 边未添加");
        
        // 验证所有预期边都存在
        let expected_edges = [
            ("a", "b", 5), ("b", "a", 5),
            ("b", "c", 10), ("c", "b", 10),
            ("c", "a", 7), ("a", "c", 7)
        ];
        
        for (f, t, w) in expected_edges.iter() {
            let f_str = &String::from(*f);
            let t_str = &String::from(*t);
            assert!(
                edges3.contains(&(f_str, t_str, *w)),
                "缺失边: ({}, {}, {})", f, t, w
            );
        }
    }

    #[test]
    fn test_add_node() {
        let mut graph = UndirectedGraph::new();
        assert!(graph.add_node("a"), "添加新节点应返回true");
        assert!(!graph.add_node("a"), "添加已有节点应返回false");
        assert!(graph.contains("a"), "节点a应存在于图中");
    }

    #[test]
    fn test_nodes() {
        let mut graph = UndirectedGraph::new();
        graph.add_node("x");
        graph.add_node("y");
        graph.add_node("z");
        
        let nodes = graph.nodes();
        assert_eq!(nodes.len(), 3, "图中应包含3个节点");
        assert!(nodes.contains(&&"x".to_string()), "节点x不存在");
        assert!(nodes.contains(&&"y".to_string()), "节点y不存在");
        assert!(nodes.contains(&&"z".to_string()), "节点z不存在");
    }
}
