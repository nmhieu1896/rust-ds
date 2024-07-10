#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut uf = UnionFind::new(n);
        println!("{:?}", uf);
        let mut res = n as i32;

        for i in 0..n {
            for j in i + 1..n {
                if is_connected[i][j] == 1 && uf.find(i) != uf.find(j) {
                    res -= 1;
                    uf.union(i, j);
                }
            }
        }
        println!("{:?}", uf);
        return res;
    }
}

#[derive(Debug)]
struct UnionFind {
    pub parent: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let rank = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        UnionFind { parent, rank }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        return self.parent[x];
    }
    pub fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] > self.rank[y] {
            self.parent[y] = x;
        } else if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            self.rank[x] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 1, 0]];
        assert_eq!(Solution::find_circle_num(is_connected), 2);
    }

    #[test]
    fn test2() {
        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(Solution::find_circle_num(is_connected), 3);
    }
}
