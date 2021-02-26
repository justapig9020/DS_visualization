use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::fs::{self, File, OpenOptions};
use std::process::Command;

#[cfg(test)]
mod misc_test {
    use super::*;
    use super::HandlerErr::*;
    use std::io::Cursor;
    type Fetched = Result<String, ()>;
    type Input = Box<dyn BufRead>;
    #[test]
    fn to_words_single() {
        let test = String::from("first");
        let got = to_words(&test);
        let expect = vec!["first"];
        assert_eq!(got.len(), 1);
        assert_eq!(got, expect);
    }
    #[test]
    fn to_words_multi() {
        let test = String::from("first second third forth");
        let got = to_words(&test);
        let expect = vec!["first", "second", "third", "forth"];
        assert_eq!(got.len(), 4);
        assert_eq!(got, expect);
    }
    fn new_input(s: &'static [u8]) -> Input{
        Box::new(Cursor::new(s))
    }
    #[test]
    fn fetch_single_word() {
        let mut input = new_input(b"normal");

        let got = fetch(&mut input);
        let _expect: Fetched =
            Ok(String::from("normal"));
        assert!(matches!(got, _expect));
    }
    #[test]
    fn fetch_multi_words() {
        let mut input = new_input(b"first second");

        let got = fetch(&mut input);
        let _expect: Fetched =
            Ok(String::from("first second"));
        assert!(matches!(got, _expect));
    }
    #[test]
    fn fetch_eof() {
        let mut input = new_input(&[]);
        let got = fetch(&mut input);
        let _expect: Fetched = Err(());
        assert!(matches!(got, _expect));
    }
    struct MockManager {
        cmd: Option<String>,
        args: Option<Vec<String>>,
        promission: bool,
    }
    impl Management for MockManager {
        fn assign_job(&mut self, cmd: &str, args: &[&str]) -> bool {
            self.cmd = Some(String::from(cmd));
            let args = args.iter()
                            .map(|s| (*s).to_string())
                            .collect();
            self.args = Some(args);
            self.promission
        }
        fn gen_graph(&self) -> String {
            String::from("")
        }
    }
    impl MockManager {
        fn new() -> Box<Self> {
            Box::new(Self{
                cmd: None,
                args: None,
                promission: true,
            })
        }
        fn get_cmds(&mut self) -> Option<String> {
            self.cmd.take()
        }
        fn get_args(&mut self) -> Option<Vec<String>> {
            self.args.take()
        }
        fn set_cmd_not_found(&mut self) {
            self.promission = false;
        }
    }
    fn init_test_string(s: Vec<&str>) -> (Vec<&str>, Vec<String>) {
        let ret = s.iter()
            .map(|s| String::from(*s))
            .collect();
        (s, ret)
    }
    #[test]
    fn handle_cmd() {
        let mut mock = MockManager::new();
        let management = &mut *mock as &mut dyn Management;
        let mut inter =
            Interactor::new(management);
        let (para, expect) =
            init_test_string(vec!["help"]);
        let got = inter.handle_cmd(para);
        assert!(matches!(got, Ok(())));
        let cmd = mock.get_cmds();
        let _expect_cmd = &expect[0];
        assert!(matches!(cmd, Some(_expect_cmd)));
    }
    #[test]
    fn handle_cmd_args() {
        let mut mock = MockManager::new();
        let management = &mut *mock as &mut dyn Management;
        let mut inter =
            Interactor::new(management);
        let (para, expect) =
            init_test_string(vec!["fist", "second", "third"]);
        let got = inter.handle_cmd(para);
        assert!(matches!(got, Ok(())));
        let cmd = mock.get_cmds();
        let _expect_cmd = &expect[0];
        assert!(matches!(cmd, Some(_expect_cmd)));
    }
    #[test]
    fn handle_empty() {
        let mut mock = MockManager::new();
        let management = &mut *mock as &mut dyn Management;
        let mut inter =
            Interactor::new(management);
        let (para, _) =
            init_test_string(vec![]);
        let got = inter.handle_cmd(para);
        assert!(matches!(got, Err(ArgLack)));
    }
    #[test]
    fn handle_exit() {
        let mut mock = MockManager::new();
        let management = &mut *mock as &mut dyn Management;
        let mut inter =
            Interactor::new(management);
        let (para, _) =
            init_test_string(vec!["exit"]);
        let got = inter.handle_cmd(para);
        assert!(matches!(got, Err(Exit)));
    }
    #[test]
    fn handle_no_cmd() {
        let mut mock = MockManager::new();
        mock.set_cmd_not_found();
        let management = &mut *mock as &mut dyn Management;
        let mut inter =
            Interactor::new(management);
        let (para, _) =
            init_test_string(vec!["notfound"]);
        let got = inter.handle_cmd(para);
        assert!(matches!(got, Err(NoCmd)));
    }
}

pub fn interactive(mut inter: Interactor, input: Option<&str>, output: Option<&str>) {
    let mut input = get_input(input);
    let mut output = Outputer::new(output);
    let got = output.create_dir();
    if let Err(e) = got {
        panic!("Create directory failed: {}", e);
    }
    loop {
        print!("> ");
        io::stdout().flush().expect("Flush stdout failed");
        let line = fetch(&mut input);
        if line.is_err() {
            break;
        }
        let line = line.unwrap();
        let words = to_words(&line);
        if let Err(e) = inter.handle_cmd(words) {
            if let HandlerErr::Exit = e {
                break;
            }
            continue;
        }
        let graph = inter.gen_graph();
        //let graph = graph.parse();
        output.write_graph(graph);
    }
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

fn fetch(input: &mut Box<dyn BufRead>) -> Result<String, ()> {
    let mut line = String::new();
    let len = input
                .read_line(&mut line)
                .expect("Read line failed");
    if len == 0 {
        // EOF
        return Err(());
    }
    Ok(line)
}

struct Outputer <'s> {
    name: Option<&'s str>,
    cnt: usize,
}

impl <'s> Outputer <'s> {
    fn new(name: Option<&'s str>) -> Self
    {
        Self {
            name,
            cnt: 0,
        }
    }
    fn create_dir(&self) -> std::io::Result<()>{
        if let Some(dir) = self.name {
            fs::create_dir_all(dir)?;
        }
        Ok(())
    }
    fn write_graph(&mut self, graph: String) {
        let (mut output, name) = get_output(&self.name, self.cnt);
        self.cnt += 1;
        output.write_fmt(format_args!("{}", graph)).expect("Write to file failed");
        drop(output);
        if let Some(name) = name {
            let outfile = format!("{}.jpg", name);
            Command::new("dot")
                    .arg("-Tjpg")
                    .arg(name.as_str())
                    .arg("-o")
                    .arg(outfile.as_str())
                    .output()
                    .expect("Generate graph failed");
        }
    }
}

fn get_output(name: &Option<&str>, i: usize) -> (Box<dyn Write>, Option<String>) {
    match name {
        Some(file) => {
            /*
            let mut name = file.to_string();
            name.push_str(&i.to_string());
            name.push_str(".gv");
            */
            let name = format!("{}/{}{}.gv", file, file, i);
            let f = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(&name)
                        .expect("Open output file failed");
            (Box::new(BufWriter::new(f)), Some(name))
        },
        None => {
            let stdout = io::stdout();
            (Box::new(BufWriter::new(stdout)), None)
        }
    }
}


fn to_words(s: &String) -> Vec<&str> {
    let split = s.split(' ');
    split.into_iter()
        .map(|s| s.trim())
        .collect()
}

pub trait Management {
    fn assign_job(&mut self, cmd: &str, args: &[&str]) -> bool;
    fn gen_graph(&self) -> String;
}

pub struct Interactor<'m> {
    manager: &'m mut dyn Management,
}

#[derive(Debug)]
enum HandlerErr {
    ArgLack,
    Exit,
    NoCmd,
}

impl <'m> Interactor <'m> {
    pub fn new(manager: &'m mut dyn Management) -> Self {
        Self {
            manager,
        }
    }
    fn handle_cmd(&mut self, args: Vec<&str>) -> Result<(), HandlerErr> {
        use HandlerErr::*;

        if args.is_empty() {
            return Err(ArgLack);
        }
        let cmd = args[0];
        if cmd == "exit" {
            return Err(Exit);
        }
        let done = self.manager.assign_job(cmd, &args[1..]);
        if done {
            Ok(())
        } else {
            self.manager.assign_job("help", &[]);
            Err(NoCmd)
        }
    }
    fn gen_graph(&self) -> String {
        self.manager.gen_graph()
    }
}
