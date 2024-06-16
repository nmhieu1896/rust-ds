#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut vis: Vec<bool> = vec![false; n];
        let mut path: Vec<bool> = vec![false; n];

        for i in 0..n {
            if vis[i] {
                continue;
            }
            Self::dfs(i, &graph, &mut vis, &mut path);
        }

        let res = path
            .iter()
            .enumerate()
            .filter_map(|(idx, &val)| if !val { Some(idx as i32) } else { None })
            .collect();

        return res;
    }
    pub fn dfs(
        cur: usize,
        graph: &Vec<Vec<i32>>,
        vis: &mut Vec<bool>,
        path: &mut Vec<bool>,
    ) -> bool {
        vis[cur] = true;
        path[cur] = true;

        for &node in graph[cur].iter() {
            // Path is a cycle, return false
            if path[node as usize] {
                return false;
            }
            // the future path will be a cycle, return false
            if !vis[node as usize] && !Self::dfs(node as usize, graph, vis, path) {
                return false;
            }
        }
        path[cur] = false;
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let graph = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![2, 4, 5, 6]);
    }

    #[test]
    fn test2() {
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![4]);
    }
}
