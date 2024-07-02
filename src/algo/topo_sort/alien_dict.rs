use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut adj: HashMap<char, HashSet<char>> = HashMap::new();
        let mut in_degree: HashMap<char, usize> = HashMap::new();

        // Build adjacency list and initialize in-degrees // bytes have better perf than chars
        for word in &words {
            for &c in word.as_bytes() {
                in_degree.entry(c as char).or_default();
            }
        }

        // Compare each word with the previous word to establish ordering rules
        for pair in words.windows(2) {
            let (first, second) = (&pair[0], &pair[1]);
            if first.len() > second.len() && first.starts_with(second) {
                return "".to_string();
            }
            for (c1, c2) in first.chars().zip(second.chars()) {
                if c1 != c2 {
                    // Only update in_degree once for a pair
                    if !adj.get(&c1).map(|x| x.contains(&c2)).unwrap_or(false) {
                        *in_degree.entry(c2).or_default() += 1;
                    }
                    adj.entry(c1).or_default().insert(c2);
                    break;
                }
            }
        }

        // Collect characters with zero in-degrees
        let mut q: VecDeque<char> = in_degree
            .iter()
            .filter(|&(_, &deg)| deg == 0)
            .map(|(&c, _)| c)
            .collect();

        let mut res = Vec::new();

        // Perform topological sort
        while let Some(c) = q.pop_front() {
            res.push(c);
            if let Some(neighbors) = adj.get(&c) {
                for &neighbor in neighbors {
                    let degree = in_degree.get_mut(&neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        q.push_back(neighbor);
                    }
                }
            }
        }

        // If result length doesn't match the number of unique characters, return ""
        if res.len() != in_degree.len() {
            return "".to_string();
        }

        res.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec!["wrt", "wrf", "er", "ett", "rftt"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let expected = "wertf";
        assert_eq!(Solution::alien_order(words), expected);
    }
    #[test]
    fn test2() {
        let words = vec!["ac", "ab", "zc", "zb"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let result = Solution::alien_order(words);

        assert!(result.starts_with("ac") || result.starts_with("ca"));
        assert!(result.ends_with("bz") || result.ends_with("zb"));
        assert!(result.len() == 4);
    }
}
