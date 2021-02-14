#[cfg(test)]
mod tests {
    use tokio_test::*;
    use super::*;

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
        while let Some(curr) = ptr {
            if curr.val == val {
                let ret = curr.val;
                *ptr = curr.next.take();
                self.size -= 1;
                return Ok(ret);
            }
        }
        Err(())
    }
    pub fn len(&self) -> usize {
        self.size
    }
}
