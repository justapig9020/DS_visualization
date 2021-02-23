use crate::graphviz::Graphviz;

#[cfg(test)]
mod tests {
    use tokio_test::*;
    use super::*;
    use std::matches;

    #[test]
    fn insert_tail() {
        let mut list = List::new();
        list.insert_tail(1);
        assert_eq!(list.size, 1);
        let node = list.head.unwrap();
        assert_eq!(node.val, 1);
    }
    #[test]
    fn insert_tail_multi() {
        let mut list = List::new();
        list.insert_tail(1);
        list.insert_tail(2);
        assert_eq!(list.size, 2);
        let node = list.head.unwrap();
        assert_eq!(node.val, 1);
        let node = node.next.unwrap();
        assert_eq!(node.val, 2);
    }
    #[test]
    fn insert_head() {
        let mut list = List::new();
        list.insert_head(1);
        assert_eq!(list.size, 1);
        let node = list.head.unwrap();
        assert_eq!(node.val, 1);
    }
    #[test]
    fn insert_head_multi() {
        let mut list = List::new();
        list.insert_head(2);
        list.insert_head(1);
        assert_eq!(list.size, 2);
        let node = list.head.unwrap();
        assert_eq!(node.val, 1);
        let node = node.next.unwrap();
        assert_eq!(node.val, 2);
    }
    #[test]
    fn remove_exist() {
        let mut list = List::new();
        list.insert_tail(1);
        let got = list.remove(1);
        assert_ok!(got);
        assert_eq!(got.unwrap(), 1);
    }
    #[test]
    fn remove_not_exist() {
        let mut list = List::new();
        let got = list.remove(1);
        assert_err!(got);
    }
    #[test]
    fn remove_multi() {
        let mut list = List::new();
        list.insert_tail(1);
        list.insert_tail(2);
        let got = list.remove(2);
        assert!(matches!(got, Ok(2)));
        assert_eq!(list.len(), 1);
    }
    #[test]
    fn list() {
        let mut list = List::new();
        list.insert_tail(1);
        list.insert_tail(2);
        list.insert_tail(3);
        list.insert_tail(4);
        list.insert_tail(5);
        let got = list.list();
        assert_eq!(got, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn find_mid_odd() {
        let mut list = List::new();
        list.insert_tail(1);
        list.insert_tail(2);
        list.insert_tail(3);
        list.insert_tail(4);
        list.insert_tail(5);
        let got = list.find_mid();
        assert!(matches!(got, Ok(3)));
    }
    #[test]
    fn find_mid_even() {
        let mut list = List::new();
        list.insert_tail(1);
        list.insert_tail(2);
        list.insert_tail(3);
        list.insert_tail(4);
        let got = list.find_mid();
        assert!(matches!(got, Ok(2)));
    }
    #[test]
    fn find_mid_empty() {
        let list = List::new();
        let got = list.find_mid();
        assert!(matches!(got, Err(())));
    }
    #[test]
    fn gen_no_node_graph() {
        let list = List::new();
        let got = list.gen_graph();
        let expect =
"digraph LinkedList {
rankdir=LR;
node [shape=record];
edge [arrowtail=dot, dir=both, tailclip=false]
len [label=\"Len | 0\"]
}".to_string();
        assert_eq!(got, expect, "\nGot:\n{}", got);
    }
    #[test]
    fn gen_one_node_graph() {
        let mut list = List::new();
        list.insert_tail(1);
        let got = list.gen_graph();
        let expect =
"digraph LinkedList {
rankdir=LR;
node [shape=record];
edge [arrowtail=dot, dir=both, tailclip=false]
len [label=\"Len | 1\"]
node1 [label=\"{<val>1 | <next>}\"]
}".to_string();
        assert_eq!(got, expect, "\nGot:\n{}", got);
    }
    #[test]
    fn gen_multi_node_graph() {
        let mut list = List::new();
        list.insert_tail(0);
        list.insert_tail(1);
        list.insert_tail(2);
        let got = list.gen_graph();
        let expect =
"digraph LinkedList {
rankdir=LR;
node [shape=record];
edge [arrowtail=dot, dir=both, tailclip=false]
len [label=\"Len | 3\"]
node0 [label=\"{<val>0 | <next>}\"]
node0:next:c -> node1;
node1 [label=\"{<val>1 | <next>}\"]
node1:next:c -> node2;
node2 [label=\"{<val>2 | <next>}\"]
}".to_string();
        assert_eq!(got, expect, "\nGot:\n{}", got);
    }
}

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

pub struct List {
    head: Option<Box<Node>>,
    size: usize,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
        }
    }
    fn forward(ptr: &Option<Box<Node>>) -> &Option<Box<Node>>{
        if let Some(curr) = ptr {
            &curr.next
        } else {
            ptr
        }
    }
}

impl Graphviz for Node {
    fn gen_graph(&self) -> String {
        let mut graph = format!("node{} [label=\"{{<val>{} | <next>}}\"]\n", self.val, self.val);
        if let Some(next) = &self.next {
            let edge = format!("node{}:next:c -> node{};\n", self.val, next.val);
            graph.push_str(&edge);
            let sub_graph = next.gen_graph();
            graph.push_str(&sub_graph);
        }
        graph
    }
}

impl List {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }
    pub fn insert_tail(&mut self, val: i32) {
        let mut ptr = &mut self.head;
        while let Some(curr) = ptr {
            ptr = &mut curr.next;
        }
        *ptr = Some(Box::new(Node::new(val)));
        self.size += 1;
    }
    pub fn insert_head(&mut self, val: i32) {
        let mut node = Box::new(Node::new(val));
        node.next = self.head.take();
        self.head = Some(node);
        self.size += 1;
    }
    pub fn remove(&mut self, val: i32) -> Result<i32, ()> {
        let mut ptr = &mut self.head;
        loop {
            match ptr {
                Some(curr)
                    if curr.val == val => {
                    let ret = curr.val;
                    *ptr = curr.next.take();
                    self.size -= 1;
                    return Ok(ret);
                },
                Some(curr) => {
                    ptr = &mut curr.next;
                },
                None => {
                    return Err(());
                },
            }
        }
    }
    pub fn list(&self) -> Vec<i32> {
        let mut ret = Vec::with_capacity(self.size);
        let mut ptr = &self.head;
        while let Some(curr) = ptr {
            ret.push(curr.val);
            ptr = &curr.next;
        }
        ret
    }
    pub fn find_mid(&self) -> Result<i32, ()> {
        let mut faster = &self.head;
        let mut slower = &self.head;
        loop {
            faster = Node::forward(faster);
            faster = Node::forward(faster);
            if faster.is_none() {
                break;
            }
            slower = Node::forward(slower);
        }
        if let Some(slow) = slower {
            Ok(slow.val)
        } else {
            Err(())
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
}

impl Graphviz for List {
    fn gen_graph(&self) -> String {
        let mut graph = String::from("digraph LinkedList {\n");
        graph += "rankdir=LR;\n";
        graph += "node [shape=record];\n";
        graph += "edge [arrowtail=dot, dir=both, tailclip=false]\n";
        graph.push_str(&format!("len [label=\"Len | {}\"]\n", self.len()));
        if let Some(curr) = &self.head {
            let nodes = curr.gen_graph();
            graph.push_str(&nodes);
        }
        graph += "}";
        graph
    }
}
