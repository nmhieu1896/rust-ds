#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut res = vec![false; queries.len()];
        let mut edge_list = edge_list;
        let mut queries = queries
            .into_iter()
            .enumerate()
            .map(|(idx, mut v)| {
                v.push(idx as i32);
                v
            })
            .collect::<Vec<Vec<i32>>>();

        edge_list.sort_by(|a, b| a[2].cmp(&b[2]));
        queries.sort_by(|a, b| a[2].cmp(&b[2]));
        let mut uf = UnionFind::new(n as usize);

        let mut edges_idx = 0;
        for q in &queries {
            while edges_idx < edge_list.len() && edge_list[edges_idx][2] < q[2] {
                uf.union(edge_list[edges_idx][0], edge_list[edges_idx][1]);
                edges_idx += 1;
            }

            res[q[3] as usize] = uf.is_connected(q[0], q[1])
        }

        return res;
    }
}

struct UnionFind {
    parent: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n + 1).collect(),
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        return self.parent[x];
    }

    pub fn union(&mut self, x: i32, y: i32) {
        let x_root = self.find(x as usize);
        let y_root = self.find(y as usize);

        match x_root >= y_root {
            true if x_root == y_root => return,
            true => self.parent[x_root] = y_root,
            _ => self.parent[y_root] = x_root,
        }
    }
    pub fn is_connected(&mut self, x: i32, y: i32) -> bool {
        self.find(x as usize) == self.find(y as usize)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test1() {
        let n = 3;
        let edges = vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]];
        let queries = vec![vec![0, 1, 2], vec![0, 2, 5]];
        assert_eq!(
            Solution::distance_limited_paths_exist(n, edges, queries),
            vec![false, true]
        );
    }
    #[test]
    fn test2() {
        let n = 5;
        let edges = vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]];
        let queries = vec![vec![0, 4, 14], vec![1, 4, 13]];
        assert_eq!(
            Solution::distance_limited_paths_exist(n, edges, queries),
            vec![true, false]
        );
    }
}
