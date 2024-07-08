#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
struct Trie {
    children: Vec<Option<Trie>>,
    is_end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl Trie {
    fn new() -> Self {
        Trie {
            children: vec![None; 26],
            is_end: false,
        }
    }

    fn insert(&mut self, word: String) {
        word.chars()
            .fold(self, |node, c| {
                node.children[c as usize - 'a' as usize].get_or_insert_with(Trie::new)
            })
            .is_end = true;
    }

    // fn get(&self, word: String) -> Option<&Trie> {
    //     word.chars().try_fold(self, |node, c| {
    //         let gg = node.children.get(c as usize - 'a' as usize)?;
    //         return gg;
    //     })
    // }

    fn search(&self, word: String) -> bool {
        let mut node = self;
        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;
            if node.children[index].is_none() {
                return false;
            }
            node = node.children[index].as_ref().unwrap();
        }
        return node.is_end;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            let index = (c as u8 - b'a') as usize;
            if node.children[index].is_none() {
                return false;
            }
            node = node.children[index].as_ref().unwrap();
        }
        return true;
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        trie.insert("ap".to_string());
        trie.insert("apple".to_string());
        println!("{:?}", trie);
        assert!(false)
    }
}
