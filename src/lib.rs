use std::{collections::HashMap, env};
const ERROR_1: &'static str = "The 'run' function has to be executed before this Function!!! Consult the Documentation for more... ERROR_CODE: '1'";
const ERROR_2: &'static str = "Consult the Documentation for more... ERROR_CODE: '2'";
#[cfg(test)]
mod tests;
struct ArgProp<'a> {
    short_name: Option<&'a str>,
    help_msg: &'a str,
    take_value: bool,
    required: bool,
}
pub enum ArgState<'a> {
    Value(&'a str),
    True,
    False,
}
pub struct Parser<'a> {
    args: HashMap<&'a str, ArgProp<'a>>,
    parsed: Option<HashMap<&'a str, Option<Option<&'a str>>>>,
    program_name: &'a str,
    usage: Option<String>,
    help: Option<String>,
}
impl<'a, 'b> Parser<'a> {
    fn arg_does_not_exist(&self, arg: &str) {
        println!(
            "{}: unrecognized option '{}'\n{}\nTry '{} --help' for more information.",
            self.program_name,
            arg,
            self.usage.as_ref().unwrap(),
            self.program_name,
        );
    }
    fn create_help(&mut self) {
        self.help = Some(String::from("Options:\n"));
        for (i, s) in self.args.iter() {
            let mut length: i8; //28 chars between help_msg and the beginning of the line
            let help = self.help.as_mut().unwrap();
            match s.short_name {
                Some(sn) => {
                    help.push_str(format!("  {}, {}", sn, i).as_str());
                    length = 22 - i.len() as i8;
                }
                None => {
                    help.push_str(format!("  {}", i).as_str());
                    length = 26 - i.len() as i8;
                }
            }
            if s.take_value {
                length -= i.len() as i8 + 1;
                help.push_str(format!("={}", i.to_uppercase()).as_str());
            }
            if length <= 2 {
                help.push_str("  ");
            } else {
                for _ in 0..length {
                    help.push(' ');
                }
            }
            help.push_str(s.help_msg);
        }
    }
    pub fn run(&mut self) {
        let mut args = env::args();
        self.create_help();
        for arg in &mut args {
            if arg == "--help" || arg == "-h" {
                println!("{}", self.help.as_ref().unwrap())
            }
            if arg.starts_with("--") {
                match self.args.get(&*arg) {
                    None => self.arg_does_not_exist(&arg),
                    Some(_) => (),
                }
            } else if arg.starts_with('-') {
            }
            match arg {
                _ => (),
            }
        }
    }
    pub fn get(&'b mut self, long_name: &'a str) -> ArgState<'b> {
        match self.parsed.as_mut().expect(ERROR_1).remove(long_name) {
            None => panic!(
                "The argument '{}' can't be requested, as it was never added!!! {}",
                long_name, ERROR_2
            ),
            Some(v) => match v {
                None => ArgState::False,
                Some(v) => match v {
                    None => ArgState::True,
                    Some(v) => ArgState::Value(v),
                },
            },
        }
    }
    pub fn new(program_name: &'a str) -> Parser<'a> {
        Parser {
            args: HashMap::new(),
            parsed: None,
            program_name,
            usage: None,
            help: None,
        }
    }
    pub fn add_argument(
        &mut self,
        long_name: &'a str,
        short_name: Option<&'a str>,
        help_msg: &'a str,
        take_value: bool,
        required: bool,
    ) {
        self.args.insert(
            long_name,
            ArgProp {
                short_name,
                help_msg,
                take_value,
                required,
            },
        );
    }
}
