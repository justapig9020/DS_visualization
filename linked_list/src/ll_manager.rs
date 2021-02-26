use crate::linked_list::List;
use interactive::Management;
use crate::graphviz::Graphviz;

pub struct LinkedListManager {
    list: List,
}

impl LinkedListManager {
    pub fn new() -> Self {
        Self {
            list: List::new(),
        }
    }
}

fn str_to_i32(strs: &[&str]) -> Vec<Result<i32, ()>> {
    let strs = strs.clone();
    let i32s: Vec<Result<i32, ()>> = strs
        .iter()
        .map(|s| s
            .parse()
            .map_err(|_| ()))
        .collect();
    i32s
}

fn do_insert_tail(list: &mut List, args: &[&str]) -> bool {
    if args.len() != 1 {
        return false;
    }
    let args = str_to_i32(args);
    if let Ok(val) = args[0] {
        list.insert_tail(val);
        true
    } else {
        false
    }
}

fn do_insert_head(list: &mut List, args: &[&str]) -> bool{
    if args.len() != 1 {
        return false;
    }
    let args = str_to_i32(args);
    if let Ok(val) = args[0] {
        list.insert_head(val);
        true
    } else {
        false
    }
}

fn do_remove(list: &mut List, args: &[&str]) -> bool {
    if args.len() != 1 {
        return false;
    }
    let args = str_to_i32(args);
    if let Ok(val) = args[0] {
        let got = list.remove(val);
        match got {
            Ok(_) => true,
            Err(_) => false,
        }
    } else {
        false
    }
}

fn do_help() -> bool{
    println!("COMMAND:");
    println!("\tit [i32]");
    println!("\tih [i32]");
    println!("\trm [i32]");
    true
}


impl Management for LinkedListManager {
    fn assign_job(&mut self, cmd: &str, args: &[&str]) -> bool {
        let list = &mut self.list;
        let ret = match cmd {
            "it" => do_insert_tail(list, args),
            "ih" => do_insert_head(list, args),
            "rm" => do_remove(list, args),
            "help" => do_help(),
            _ => false,
        };
        ret
    }
    fn gen_graph(&self) -> String {
        self.list.gen_graph()
    }
}
