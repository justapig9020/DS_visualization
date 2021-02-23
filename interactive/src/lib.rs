use std::io::{self, BufRead, BufReader, Write, BufWriter};
use std::fs::{File, OpenOptions};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_words_single() {
        let test = String::from("first");
        let got = to_words(test.as_str());
        let expect = vec!["first"];
        assert_eq!(got.len(), 1);
        assert_eq!(got, expect);
    }
    #[test]
    fn to_words_multi() {
        let test = String::from("first second third forth");
        let got = to_words(test.as_str());
        let expect = vec!["first", "second", "third", "forth"];
        assert_eq!(got.len(), 4);
        assert_eq!(got, expect);
    }
}

pub fn interactive(inter: Interactor, input: Option<&str>, output: Option<&str>) {
    let input = get_input(input);
    /*
    let mut output = Outputer::new(output);
    loop {
        let words = fetch(input);
        inter.handle_cmd(words);
        let graph = inter.gen_graph();
        let graph = graph.parse();
        output.write_graph(graph);
    }
    */
}

fn get_input(name: Option<&str>) -> Box<dyn BufRead> {
    match name {
        Some(file) => {
            let f = File::open(file).expect("Open input file failed");
            Box::new(BufReader::new(f))
        },
        None => {
            let stdin = io::stdin();
            Box::new(BufReader::new(stdin))
        }
    }
}

struct Outputer <'s> {
    name: &'s str,
    cnt: usize,
}

impl <'s> Outputer <'s> {
    fn new(name: Option<&'s str>) -> Self
    {
        Self {
            name: name.unwrap_or("stdout"),
            cnt: 0,
        }
    }
    fn write_graph(graph: String) {
    }
}

fn to_words<'s, 'v> (s: &'s str) -> Vec<&'v str>
    where 's: 'v
{
    let iter = s.split(' ');
    iter.collect()
}

pub trait Management {
    fn assign_job(&mut self, cmd: &str, args: Vec<&str>);
    fn gen_graph(&self) -> String;
}

pub struct Interactor {
    manager: Box<dyn Management>,
}

impl Interactor {
    fn new(manager: Box<dyn Management>) -> Self {
        Self {
            manager
        }
    }
}
