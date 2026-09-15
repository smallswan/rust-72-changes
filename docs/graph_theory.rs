//! # 图论模块
//!
//! 对应第五十八章，涵盖图遍历、最短路径、最小生成树、拓扑排序等。

use std::collections::VecDeque;

/// 无向加权图（邻接表表示）
#[derive(Debug, Clone)]
pub struct Graph {
    pub n: usize,
    pub adj: Vec<Vec<(usize, u64)>>, // (邻居节点, 权重)
}

impl Graph {
    /// 创建 n 个顶点的空图
    pub fn new(n: usize) -> Self {
        Graph {
            n,
            adj: vec![Vec::new(); n],
        }
    }

    /// 添加无向边
    pub fn add_edge(&mut self, u: usize, v: usize, w: u64) {
        self.adj[u].push((v, w));
        self.adj[v].push((u, w));
    }

    /// 添加有向边
    pub fn add_directed_edge(&mut self, u: usize, v: usize, w: u64) {
        self.adj[u].push((v, w));
    }

    /// 广度优先搜索，返回从 start 出发的访问顺序
    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &(v, _) in &self.adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }
        order
    }

    /// 广度优先搜索求最短距离（无权图）
    pub fn bfs_shortest_path(&self, start: usize) -> Vec<i64> {
        let mut dist = vec![-1i64; self.n];
        let mut queue = VecDeque::new();
        dist[start] = 0;
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            for &(v, _) in &self.adj[u] {
                if dist[v] == -1 {
                    dist[v] = dist[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        dist
    }

    /// 深度优先搜索（迭代式，避免栈溢出）
    pub fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        let mut stack = vec![start];

        while let Some(u) = stack.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            order.push(u);
            for &(v, _) in self.adj[u].iter().rev() {
                if !visited[v] {
                    stack.push(v);
                }
            }
        }
        order
    }

    /// Dijkstra 最短路径（非负权边），返回从 start 到各点的最短距离
    pub fn dijkstra(&self, start: usize) -> Vec<u64> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut dist = vec![u64::MAX; self.n];
        let mut heap = BinaryHeap::new();

        dist[start] = 0;
        heap.push(Reverse((0u64, start)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &self.adj[u] {
                let new_dist = dist[u].saturating_add(w);
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    heap.push(Reverse((new_dist, v)));
                }
            }
        }
        dist
    }
}

/// 并查集（Union-Find），支持路径压缩和按秩合并
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // 路径压缩
        }
        self.parent[x]
    }

    /// 合并两个集合，返回是否成功合并（原本不同集合）
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (rx, ry) = (self.find(x), self.find(y));
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
        true
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

/// Kruskal 最小生成树算法，返回最小总权重
pub fn kruskal_mst(n: usize, edges: &mut Vec<(usize, usize, u64)>) -> u64 {
    edges.sort_by_key(|e| e.2);
    let mut uf = UnionFind::new(n);
    let mut total_weight = 0u64;

    for &(u, v, w) in edges.iter() {
        if uf.union(u, v) {
            total_weight += w;
        }
    }
    total_weight
}

/// 拓扑排序（Kahn 算法），返回 None 表示有环
pub fn topological_sort(n: usize, adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let mut in_degree = vec![0usize; n];
    for neighbors in adj {
        for &v in neighbors {
            in_degree[v] += 1;
        }
    }

    let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut order = Vec::with_capacity(n);

    while let Some(u) = queue.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    if order.len() == n { Some(order) } else { None }
}

/// Floyd-Warshall 全源最短路径
pub fn floyd_warshall(n: usize, edges: &[(usize, usize, u64)]) -> Vec<Vec<u64>> {
    let mut dist = vec![vec![u64::MAX; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for &(u, v, w) in edges {
        dist[u][v] = dist[u][v].min(w);
    }
    for k in 0..n {
        for i in 0..n {
            if dist[i][k] == u64::MAX {
                continue;
            }
            for j in 0..n {
                if dist[k][j] == u64::MAX {
                    continue;
                }
                let new_dist = dist[i][k].saturating_add(dist[k][j]);
                dist[i][j] = dist[i][j].min(new_dist);
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_dfs() {
        let mut g = Graph::new(6);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(1, 3, 1);
        g.add_edge(2, 4, 1);
        g.add_edge(3, 5, 1);

        let bfs_order = g.bfs(0);
        assert_eq!(bfs_order[0], 0);
        assert!(bfs_order.contains(&5));

        let dfs_order = g.dfs(0);
        assert_eq!(dfs_order[0], 0);
        assert!(dfs_order.contains(&5));
    }

    #[test]
    fn test_bfs_shortest_path() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(1, 3, 1);
        g.add_edge(2, 3, 1);
        g.add_edge(3, 4, 1);

        let dist = g.bfs_shortest_path(0);
        assert_eq!(dist[0], 0);
        assert_eq!(dist[1], 1);
        assert_eq!(dist[3], 2);
        assert_eq!(dist[4], 3);
    }

    #[test]
    fn test_dijkstra() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1, 10);
        g.add_edge(0, 2, 3);
        g.add_edge(1, 3, 2);
        g.add_edge(2, 1, 4);
        g.add_edge(2, 3, 8);
        g.add_edge(3, 4, 9);

        let dist = g.dijkstra(0);
        assert_eq!(dist[0], 0);
        assert_eq!(dist[1], 7); // 0->2->1
        assert_eq!(dist[2], 3);
        assert_eq!(dist[3], 9); // 0->2->1->3
        assert_eq!(dist[4], 18);
    }

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(5);
        assert!(uf.union(0, 1));
        assert!(uf.union(2, 3));
        assert!(uf.connected(0, 1));
        assert!(uf.connected(2, 3));
        assert!(!uf.connected(0, 2));
        assert!(!uf.union(0, 1)); // 已同集合
    }

    #[test]
    fn test_kruskal() {
        let mut edges = vec![(0, 1, 4), (0, 2, 1), (1, 2, 2), (1, 3, 3), (2, 3, 5)];
        let weight = kruskal_mst(4, &mut edges);
        assert_eq!(weight, 6); // 边 (0,2,1)+(1,2,2)+(1,3,3)=6
    }

    #[test]
    fn test_topological_sort() {
        let n = 4;
        let adj = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let order = topological_sort(n, &adj).unwrap();
        // 验证拓扑序：0 在 1、2 前面，1 和 2 在 3 前面
        let pos: std::collections::HashMap<usize, usize> =
            order.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        assert!(pos[&0] < pos[&1]);
        assert!(pos[&0] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }

    #[test]
    fn test_topological_sort_with_cycle() {
        let n = 3;
        let adj = vec![vec![1], vec![2], vec![0]]; // 0->1->2->0 环
        assert!(topological_sort(n, &adj).is_none());
    }

    #[test]
    fn test_floyd_warshall() {
        let edges = vec![(0, 1, 3), (1, 2, 1), (0, 2, 10)];
        let dist = floyd_warshall(3, &edges);
        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 3);
        assert_eq!(dist[0][2], 4); // 0->1->2
        assert_eq!(dist[1][2], 1);
    }
}
