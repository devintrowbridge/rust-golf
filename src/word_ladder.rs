use std::{collections::{VecDeque, HashSet}, rc::Rc};

#[derive(Clone, Debug)]
struct Node<T> {
    pub val : T,
    parent: Option<Rc<Node<T>>>,
}

impl<T> Node<T> 
where T : Clone
{
    pub fn new(val: T) -> Node<T> {
        Node {
            val,
            parent: None,
        }
    }

    pub fn add_child(&self, child: T) -> Rc<Node<T>> {
        let mut child = Self::new(child);
        child.parent = Some(Rc::new(self.clone()));
        Rc::new(child)
    }

    pub fn path_to_root(&self) -> Vec<T> {
        let mut path = Vec::new();
        path.push(self.val.clone());

        let mut current = self.parent.clone().unwrap();
        while current.parent.is_some() {
            path.push(current.val.clone());
            current = current.parent.clone().unwrap();
        }
        path.push(current.val.clone());

        path
    }
}

pub fn hamming_distance(lhs: &str, rhs: &str) -> Option<u32> {
    if lhs.len() != rhs.len() { return None }

    let mut distance = 0;
    for (lchar, rchar) in lhs.chars().zip(rhs.chars()) {
        if lchar != rchar { distance += 1; }
    }

    Some(distance)
}

pub fn get_neighbors(word: &str, word_list: &[String]) -> Vec<String> {
    let mut neighbors = Vec::new();
    
    for el in word_list {
        if let Some(1) = hamming_distance(word, el) {
            neighbors.push(el.to_string());
        }
    }

    neighbors
}

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut touched = HashSet::new();

    if begin_word.len() != end_word.len() { return 0 }
    if begin_word == end_word { 
        return 0;
    }

    touched.insert(begin_word.clone());
    let mut bfs_queue = VecDeque::new();
    bfs_queue.push_back(Rc::new(Node::new(begin_word)));

    while !bfs_queue.is_empty() {
        // unwrap is ok here since we verify Some via loop conditions 
        let current = bfs_queue.pop_front().unwrap();

        if current.val == end_word {
            return current.path_to_root().len() as i32;
        }

        for neighbor in get_neighbors(&current.val, &word_list) {
            if touched.insert(neighbor.clone()) {
                let neighbor = current.add_child(neighbor);
                bfs_queue.push_back(neighbor);
            }
        }
    }

    0 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let begin_word = "hit".into();
        let end_word = "cog".into();
        let word_list = vec!["hot".into(),"dot".into(),"dog".into(),"lot".into(),"log".into(),"cog".into()];

        let ladder_len = ladder_length(begin_word, end_word, word_list);
        assert_eq!(ladder_len, 5);
    }
}