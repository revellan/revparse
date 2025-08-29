use std::{collections::HashMap, env};
const ERROR_1: &'static str = "The 'run' function has to be executed before this Function!!! Consult the Documentation for more... ERROR_CODE: '1'";
const ERROR_2: &'static str = "Consult the Documentation for more... ERROR_CODE: '2'";

#[cfg(test)]
mod tests;
struct ArgProp<'a> {
    short_name: Option<&'a str>,
    help_msg: &'a str,
    take_value: bool,
}
pub enum ArgState<'a> {
    Value(&'a str),
    True,
    False,
}
pub struct Parser<'a> {
    args: HashMap<&'a str, ArgProp<'a>>,
    parsed: Option<HashMap<&'a str, Option<Option<&'a str>>>>,
}
impl<'a, 'b> Parser<'a> {
    pub fn run(&self) {
        let args = env::args();
        for arg in args {
            if arg.starts_with("--") {
                for i in <
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
    pub fn new() -> Parser<'a> {
        Parser {
            args: HashMap::new(),
            parsed: None,
        }
    }
    pub fn add_argument(
        &mut self,
        long_name: &'a str,
        short_name: Option<&'a str>,
        help_msg: &'a str,
        take_value: bool,
    ) {
        self.args.insert(
            long_name,
            ArgProp {
                short_name,
                help_msg,
                take_value,
            },
        );
    }
}
