use std::collections::VecDeque;

// 定义图结构
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // 创建具有n个顶点的新图
    fn new(n: usize) -> Self {
        Graph { adj: vec![vec![]; n] }
    }

    // 向图中添加边（无向图）
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // 执行广度优先搜索，返回访问节点的顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 检查起始节点是否有效
        if start >= self.adj.len() {
            return vec![];
        }

        // 初始化访问标记数组
        let mut visited = vec![false; self.adj.len()];
        // 初始化队列并加入起始节点
        let mut queue = VecDeque::new();
        // 初始化访问顺序向量
        let mut visit_order = Vec::new();

        // 标记起始节点为已访问并加入队列
        visited[start] = true;
        queue.push_back(start);

        // 当队列不为空时继续搜索
        while let Some(node) = queue.pop_front() {
            // 将当前节点加入访问顺序
            visit_order.push(node);

            // 获取当前节点的所有邻居并排序（确保访问顺序一致）
            let mut neighbors = self.adj[node].clone();
            neighbors.sort_unstable();  // 排序邻居以保证一致的访问顺序

            // 遍历所有邻居
            for &neighbor in &neighbors {
                if !visited[neighbor] {
                    // 标记为已访问并加入队列
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);
        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}