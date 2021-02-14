use crate::graphviz::Graphviz;

#[cfg(test)]
mod tests {
    use tokio_test::*;
    use super::*;
    use std::matches;

    #[test]
    fn add_node() {
        let mut list = List::new();
        list.insert_tail(1);
        assert_eq!(list.size, 1);
        let node = list.head.unwrap();
        assert_eq!(node.val, 1);
    }
    #[test]
    fn add_multi_nodes() {
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
    fn del_exist_node() {
        let mut list = List::new();
        list.insert_tail(1);
        let got = list.remove(1);
        assert_ok!(got);
        assert_eq!(got.unwrap(), 1);
    }
    #[test]
    fn del_not_exist_node() {
        let mut list = List::new();
        let got = list.remove(1);
        assert_err!(got);
    }
    #[test]
    fn del_multi_node() {
        let mut list = List::new();
        list.insert_tail(1);
        list.insert_tail(2);
        let got = list.remove(2);
        assert!(matches!(got, Ok(2)));
        assert_eq!(list.len(), 1);
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
}

impl Graphviz for Node {
    fn gen_graph(&self) -> String {
        format!("node{} [label=\"{{<val>{} | <next>}}\"]\n", self.val, self.val)
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
        let mut ptr = &self.head;
        let mut pre_node = None;
        loop {
            if let Some(curr) = ptr {
                let node = curr.gen_graph();
                let mut vec: Vec<&str> = node.splitn(2, " ").collect();
                let curr_node = vec.remove(0);
                let curr_node = curr_node.to_string();
                if let Some(last_node) = pre_node {
                    graph.push_str(&format!("{}:next:c -> {};\n", last_node, curr_node));
                }
                pre_node = Some(curr_node);
                //pre_node = Some(last_node);
                graph.push_str(&node);
                ptr = &curr.next;
            } else {
                break;
            }
        }
        graph += "}";
        graph
    }
}
