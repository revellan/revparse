use std::{collections::HashMap, env, process};
const ERROR_1: &'static str = "The 'run' function has to be executed before this Function!!! Consult the Documentation for more... ERROR_CODE: '1'";
const ERROR_2: &'static str = "Consult the Documentation for more... ERROR_CODE: '2'";
#[cfg(test)]
mod tests;
struct ArgProp<'a> {
    short_name: Option<&'a str>,
    help_msg: &'a str,
    take_value: Option<&'a str>,
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
        self.usage = Some(String::from(format!(
            "Usage: {} [OPTION]...",
            self.program_name
        )));
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
            if s.take_value.is_some() {
                length -= i.len() as i8 + 1;
                help.push_str(format!("={}", s.take_value.unwrap().to_uppercase()).as_str());
            }
            if length <= 2 {
                help.push_str("  ");
            } else {
                for _ in 0..length {
                    help.push(' ');
                }
            }
            help.push_str(s.help_msg);
            help.push('\n');
        }
    }
    fn print_help(&self) {
        println!(
            "{}\n\n\n{}",
            self.usage.as_ref().unwrap(),
            self.help.as_ref().unwrap()
        );
        process::exit(0);
    }
    pub fn run(&mut self) {
        let mut args = env::args();
        self.create_help();
        let mut _next_is_val = false;
        self.parsed = Some(HashMap::new());
        let parsed = self.parsed.unwrap();
        for (arg, prop) in &self.args {
            args.nth(0);
            loop {
                let env_arg = args.next();
                match env_arg {
                    None => break,
                    Some(env_arg) => {
                        if env_arg == "--help" || env_arg == "-h" {
                            self.print_help();
                        } else if env_arg.starts_with("--") {
                            let splitted_arg = env_arg.split_once('=');
                            match splitted_arg {
                                Some((name,value)) => {
                                    if name == *arg {
                                        parsed.insert(name, Some())
                                    }
                                }
                                None => (),
                            }
                        } else if env_arg.starts_with('-') {
                        }
                    }
                }
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
        take_value: Option<&'a str>,
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
