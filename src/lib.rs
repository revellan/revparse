use std::{collections::HashMap, env, process};
const ERROR_1: &'static str = "The 'run' function has to be executed before this Function!!! Consult the Documentation for more... ERROR_CODE: '1'";
//const ERROR_2: &'static str = "Consult the Documentation for more... ERROR_CODE: '2'";
#[cfg(test)]
mod tests;
struct ArgProp<'a> {
    short_name: Option<&'a str>,
    help_msg: &'a str,
    take_value: Option<&'a str>,
    //required: bool,
}
pub enum ArgState {
    Value(String),
    True,
    False,
}
pub struct Parser<'a> {
    args: HashMap<&'a str, ArgProp<'a>>,
    parsed: Option<HashMap<String, Option<String>>>,
    program_name: &'a str,
    usage: Option<String>,
    help: Option<String>,
}
impl<'a, 'b> Parser<'a> {
    fn arg_does_not_exist(&self, arg: &str) {
        if arg == "--help" || arg == "-h" {
            self.no_val_allowed(arg);
        } else {
            println!(
                "{}: unrecognized option '{}'\n{}\nTry '{} --help' for more information.",
                self.program_name,
                arg,
                self.usage.as_ref().unwrap(),
                self.program_name,
            );
        }
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
    }
    fn no_val_allowed(&self, arg: &str) {
        println!(
            "{}: option '{}' doesn't allow an argument\n{}\nTry '{} --help' for more information.",
            self.program_name,
            arg,
            self.usage.as_ref().unwrap(),
            self.program_name,
        );
    }
    pub fn run(&mut self) {
        let args = env::args();
        self.create_help();
        let mut next_is_val: Option<String> = None;
        self.parsed = Some(HashMap::new());
        let parsed = self.parsed.as_mut().unwrap();
        'outer: for e_arg in args {
            if next_is_val.is_some() {
                parsed.insert(next_is_val.unwrap(), Some(e_arg));
                next_is_val = None;
                continue 'outer;
            }
            if e_arg == "--help" || e_arg == "-h" {
                self.print_help();
                process::exit(0);
            }
            if e_arg.starts_with("--") {
                match e_arg
                    .split_once('=')
                    .map(|(arg_name, val)| (arg_name.to_string(), val.to_string()))
                {
                    Some((arg_name, val)) => {
                        for (p_arg, prop) in &self.args {
                            if arg_name == *p_arg {
                                if prop.take_value.is_some() {
                                    parsed.insert(arg_name, Some(val));
                                    continue 'outer;
                                } else {
                                    self.no_val_allowed(&arg_name);
                                    process::exit(0);
                                }
                            }
                        }
                        self.arg_does_not_exist(&arg_name);
                        process::exit(0);
                    }
                    None => {
                        for (p_arg, prop) in &self.args {
                            if e_arg == *p_arg {
                                if prop.take_value.is_some() {
                                    next_is_val = Some(e_arg);
                                    continue 'outer;
                                } else {
                                    parsed.insert(e_arg, None);
                                    continue 'outer;
                                }
                            }
                        }
                        self.arg_does_not_exist(&e_arg);
                        process::exit(0);
                    }
                }
            }
        }
    }
    pub fn get(&mut self, long_name: &'a str) -> ArgState {
        match self.parsed.as_mut().expect(ERROR_1).remove(long_name) {
            //None => panic!(
            //"The argument '{}' can't be requested, as it was never added!!! {}",
            //long_name, ERROR_2
            //),
            //Some(v) => match v {
            None => ArgState::False,
            Some(v) => match v {
                None => ArgState::True,
                Some(v) => ArgState::Value(v),
            },
            //},
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
        //required: bool,
    ) {
        self.args.insert(
            long_name,
            ArgProp {
                short_name,
                help_msg,
                take_value,
                //required,
            },
        );
    }
}
