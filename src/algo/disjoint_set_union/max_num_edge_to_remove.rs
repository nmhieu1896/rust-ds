#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut alice = UnionFind::new(n as usize);
        let mut bob = UnionFind::new(n as usize);

        let mut edge_count = 0;
        for edge in &edges {
            if edge[0] == 3 {
                edge_count += (alice.union(edge[1], edge[2])) & (bob.union(edge[1], edge[2]));
            }
        }

        for edge in &edges {
            edge_count += match edge[0] {
                1 => alice.union(edge[1], edge[2]),
                2 => bob.union(edge[1], edge[2]),
                _ => 0,
            }
        }

        if alice.is_connected() && bob.is_connected() {
            return edges.len() as i32 - edge_count;
        }

        return -1;
    }
}

struct UnionFind {
    parent: Vec<usize>,
    pub n: usize,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n + 1).collect(),
            n,
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        return self.parent[x];
    }

    pub fn union(&mut self, x: i32, y: i32) -> i32 {
        let x_root = self.find(x as usize);
        let y_root = self.find(y as usize);
        println!("x_root: {:?}, y_root: {:?}", x_root, y_root);

        match x_root >= y_root {
            true if x_root == y_root => return 0,
            true => self.parent[x_root] = y_root,
            _ => self.parent[y_root] = x_root,
        }
        self.n -= 1;
        return 1;
    }

    pub fn is_connected(&self) -> bool {
        return self.n == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let n = 4;
        let edges = vec![
            vec![3, 1, 2],
            vec![3, 2, 3],
            vec![1, 1, 3],
            vec![1, 2, 4],
            vec![1, 1, 2],
            vec![2, 3, 4],
        ];
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), 2);
    }
    #[test]
    fn test2() {
        let n = 4;
        let edges = vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]];
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), 0);
    }

    #[test]
    fn test3() {
        let n = 4;
        let edges = vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]];
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), -1);
    }
}
