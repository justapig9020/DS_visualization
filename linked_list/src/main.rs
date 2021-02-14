mod linked_list;
mod graphviz;

use linked_list::List;
use std::io::{self, BufRead, BufReader, Write, BufWriter};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use graphviz::Graphviz;
use clap::{App, Arg};
use std::fs::{File, OpenOptions};

#[cfg(test)]
mod read_i32_test {
    use super::read_i32;
    use std::io::{Cursor, BufRead};

    #[test]
    fn get_i32() {
        let mut input: Box<dyn BufRead> = Box::new(Cursor::new(b"123"));
        let got = read_i32(&mut input);
        assert_eq!(got, 123);
    }
}

#[cfg(test)]
mod read_cmd_test {
    use std::matches;
    use std::io::{Cursor, BufRead};
    use super::{read_cmd, CMD};

    #[test]
    fn get_ins() {
        let mut input: Box<dyn BufRead> = Box::new(Cursor::new(b"ins\n123"));
        let got = read_cmd(&mut input);
        assert!(matches!(got, CMD::INS(123)), "Got: {:?}", got);
    }
    #[test]
    fn get_rm() {
        let mut input: Box<dyn BufRead> = Box::new(Cursor::new(b"rm\n123"));
        let got = read_cmd(&mut input);
        assert!(matches!(got, CMD::RM(123)), "Got: {:?}", got);
    }
    #[test]
    fn get_len() {
        let mut input: Box<dyn BufRead> = Box::new(Cursor::new(b"len"));
        let got = read_cmd(&mut input);
        assert!(matches!(got, CMD::LEN));
    }
    #[test]
    fn get_help() {
        let mut input: Box<dyn BufRead> = Box::new(Cursor::new(b"help"));
        let got = read_cmd(&mut input);
        assert!(matches!(got, CMD::HELP), "Got: {:?}", got);
    }
    #[test]
    fn get_none() {
        let mut input: Box<dyn BufRead> = Box::new(Cursor::new(b"nothing"));
        let got = read_cmd(&mut input);
        assert!(matches!(got, CMD::NONE), "Got: {:?}", got);
    }
    #[test]
    fn get_exit() {
        let mut input: Box<dyn BufRead> = Box::new(Cursor::new(b"exit"));
        let got = read_cmd(&mut input);
        assert!(matches!(got, CMD::EXIT), "Got: {:?}", got);
    }
}

fn main() {
    let prog = App::new("LinkList")
                    .arg(Arg::with_name("input")
                            .help("input script")
                            .short("i")
                            .long("input")
                            .takes_value(true))
                    .arg(Arg::with_name("output")
                            .help("output file")
                            .short("o")
                            .long("output")
                            .takes_value(true))
                    .get_matches();
    let reader = get_input(prog.value_of("input"));
    let writer = match prog.value_of("output") {
        Some(name) => Some(name.to_string()),
        None => None,
    };
    shell(&mut List::new(), reader, writer);
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
        },
    }
}

fn get_output(name: &Option<String>, i: usize) -> Box<dyn Write> {
    match name {
        Some(file) => {
            let mut name = file.clone();
            name.push_str(&i.to_string());
            name.push_str(".gv");
            let f = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(&name)
                        .expect("Open output file failed");
            Box::new(BufWriter::new(f))
        },
        None => {
            let stdout = io::stdout();
            Box::new(BufWriter::new(stdout))
        }
    }
}

fn shell(list: &mut List, mut input: Box<dyn BufRead>, output: Option<String>) {
    let mut i = 0;
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        match read_cmd(&mut input) {
            CMD::INS(val) => list.insert_tail(val),
            CMD::RM(val) => {
                match list.remove(val) {
                    Ok(val) => println!("{} removed", val),
                    Err(_) => println!("Remove failed"),
                }
            },
            CMD::LEN => println!("Len: {}", list.len()),
            CMD::HELP | CMD::NONE => do_help(),
            CMD::EXIT => break,
        }
        let graph = list.gen_graph();
        let mut out = get_output(&output, i);
        i += 1;
        out.write_fmt(format_args!("{}", graph)).expect("Write to file failed");
    }
}

fn do_help() {
    println!("Help:");
    for cmd in CMD::iter() {
        println!("    {:?}", cmd);
    }
}

#[derive(Debug, EnumIter)]
enum CMD {
    INS(i32),
    RM(i32),
    LEN,
    HELP,
    NONE,
    EXIT,
}

fn read_cmd(reader: &mut Box<dyn BufRead>) -> CMD {
    let mut cmd = String::new();
    let len = reader.read_line(&mut cmd).expect("Read line failed");
    if len == 0 {
        // got EOF
        return CMD::EXIT;
    }
    match cmd.trim() {
        "ins" => CMD::INS(read_i32(reader)),
        "rm"  => CMD::RM(read_i32(reader)),
        "help" => CMD::HELP,
        "exit" => CMD::EXIT,
        "len" => CMD::LEN,
        _ => CMD::NONE,
    }
}

fn read_i32(reader: &mut Box<dyn BufRead>) -> i32 {
    let mut num = String::new();
    loop {
        reader.read_line(&mut num).expect("Read line failed");
        if let Ok(ret) = num.trim().parse() {
            return ret;
        }
    }
}
