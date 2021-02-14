mod linked_list;
mod graphviz;

use linked_list::List;
use std::io::{self, BufRead, Write};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use graphviz::Graphviz;

#[cfg(test)]
mod read_i32_test {
    use super::read_i32;

    #[test]
    fn get_i32() {
        let input = b"123";
        let got = read_i32(&input[..]);
        assert_eq!(got, 123);
    }
}

#[cfg(test)]
mod read_cmd_test {
    use std::matches;
    use super::{read_cmd, CMD};

    #[test]
    fn get_ins() {
        let input = b"ins\n123";
        let got = read_cmd(&input[..]);
        assert!(matches!(got, CMD::INS(123)), "Got: {:?}", got);
    }
    #[test]
    fn get_rm() {
        let input = b"rm\n123";
        let got = read_cmd(&input[..]);
        assert!(matches!(got, CMD::RM(123)), "Got: {:?}", got);
    }
    #[test]
    fn get_len() {
        let input = b"len";
        let got = read_cmd(&input[..]);
        assert!(matches!(got, CMD::LEN));
    }
    #[test]
    fn get_help() {
        let input = b"help";
        let got = read_cmd(&input[..]);
        assert!(matches!(got, CMD::HELP), "Got: {:?}", got);
    }
    #[test]
    fn get_none() {
        let input = b"nothing";
        let got = read_cmd(&input[..]);
        assert!(matches!(got, CMD::NONE), "Got: {:?}", got);
    }
    #[test]
    fn get_exit() {
        let input = b"exit";
        let got = read_cmd(&input[..]);
        assert!(matches!(got, CMD::EXIT), "Got: {:?}", got);
    }
}

fn main() {
    shell(&mut List::new());
}

fn shell(list: &mut List) {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let input = stdin.lock();
        match read_cmd(input) {
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
        println!("{}", graph);
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

fn read_cmd<R>(mut reader: R) -> CMD
where
    R: BufRead
{
    let mut cmd = String::new();
    reader.read_line(&mut cmd).expect("Read line failed");
    match cmd.trim() {
        "ins" => CMD::INS(read_i32(reader)),
        "rm"  => CMD::RM(read_i32(reader)),
        "help" => CMD::HELP,
        "exit" => CMD::EXIT,
        "len" => CMD::LEN,
        _ => CMD::NONE,
    }
}

fn read_i32<R>(mut reader: R) -> i32
where
    R: BufRead
{
    let mut num = String::new();
    loop {
        reader.read_line(&mut num).expect("Read line failed");
        if let Ok(ret) = num.trim().parse() {
            return ret;
        }
    }
}
