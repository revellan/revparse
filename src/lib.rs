use std::{collections::HashMap, env};
const ERROR_1: &'static str = "The 'run' function has to be executed before this Function!!! Consult the Documentation for more... ERROR_CODE: '1'";
#[cfg(test)]
mod tests;
#[cfg(test)]
fn exit(code: i32) -> ! {
    panic!("Exit code: '{}'", code);
}
#[cfg(not(test))]
fn exit(code: i32) -> ! {
    std::process::exit(code);
}
struct ArgProp<'a> {
    short_name: Option<&'a str>,
    help_msg: &'a str,
    take_value: Option<&'a str>,
}
pub enum ArgState {
    Value(String),
    True,
    False,
}
pub struct Parser<'a> {
    args: Vec<(&'a str, ArgProp<'a>)>,
    parsed: Option<HashMap<String, Option<String>>>,
    program_name: &'a str,
    usage: Option<String>,
    help: Option<String>,
}
impl<'a, 'b> Parser<'a> {
    //! The Parser struct is the Heart of revparse.
    //! Here is a brief explanation.
    //! # Usage
    //! First you have to create an instance of the Parser struct and provide the name of your Program, that will later be used for those cases:
    //! ```txt
    //! your_program_name: unrecognized option '-a'
    //! Usage: your_program_name [OPTION]...
    //! Try 'your_program_name --help' for more information.
    //! ```
    //! You can create an instance of Parser by calling the associated new() function with your programs name as an argument and assigning the returned Parser instance to a mutable variable (it has to be mutable!):
    //! ```rust
    //! use revparse::Parser;
    //! let mut parser = Parser::new("your_program_name");
    //! ```
    //! To add arguments, you can use the .add_argument() function on parser.
    //! The function takes 4 Parameters apart from self.
    //!
    //! The First is the long name, that has to start with "--" and is required, not optional.
    //!
    //! The Second is an optional short name, of type Option<&str>. If it is set to None, there will be no short name for that argument, if you want a short name, like "-e" you will have to wrap it in Some() like this Some("-e"). Short names have to start with a '-' and only contain one other character.
    //!
    //! The Third option is the help message, that will be shown behind the corresponding option, when --help is called.
    //!
    //! The Fourth options is about wheter the argument can take values, or arguments like this:
    //! ```sh
    //! your_program_name --option-that-takes-a-value="This is the value"
    //! your_program_name --option-that-takes-a-value "This is the value"
    //! your_program_name -o"This is the value"
    //! your_program_name -o "This is the value"
    //! ```
    //! If you want this to be possible, you have to provide a name for the value to be shown in the help message wrapped in a Some().
    //! For example to add an argument "--start-process" that takes a value "PROCESS" you have to write the following:
    //! ```rust
    //! use revparse::Parser;
    //! let mut parser = Parser::new("your_program_name");
    //! parser.add_argument("--start-process", Some("-s"), "Start some process, this is the help message", Some("PROCESS"));
    //! ```
    //! You don't have to provide "PROCESS" in capital letters, since they will be capitalized automatically. This is what "PROCESS" is needed for:
    //! ```txt
    //! Usage: your_program_name [OPTION]...
    //!
    //! Options:
    //!   -s, --start-process=PROCESS  Start some process, this is the help message
    //!   ^-2 ^-1.parameter   ^-4.p.   ^-3.parameter
    //! ```
    //!
    //! To get the value of the arguments, you can use the .get() function defined on Parser. But before you can do that, you'll have to call .run():
    //! ```rust
    //! use revparse::Parser;
    //! let mut parser = Parser::new("your_program_name");
    //! parser.add_argument("--start-process", Some("-s"), "Start some process, this is the help message", Some("PROCESS"));
    //! parser.run();
    //! ```
    //!
    //! Then you can call the .get() function on parser and provide the long name of your argument as a function parameter, which will return an enum called ArgState with three possible variants:
    //!
    //! True,
    //! False,
    //! Value(String)
    //!
    //! True will be returned, if the argument doesn't require a value to be inserted into it, as with --start-process="Value" and was called.
    //! False will be returned, if the argument wasn't called, no matter wheter a value is needed or not.
    //! Value(String) will be returned, if the argument needs a value, and was called with one. You are given ownership of the returned String.
    //!
    //! You can best handle ArgState with a match expression like this:
    //! ```rust
    //! use revparse::{ArgState, Parser};
    //! let mut parser = Parser::new("your_program_name");
    //! parser.add_argument("--start-process", Some("-s"), "Start some process, this is the help message", Some("PROCESS"));
    //! parser.run();
    //! let result: ArgState = parser.get("--start-process");
    //! match result {
    //!     ArgState::True => panic!("Impossible"), // True will only be the case, if you didn't allow a value
    //!     ArgState::False => println!("Argument '--start-process' was not called"),
    //!     ArgState::Value(value) => println!("Argument '--start-process' was called with the value: '{value}'"),
    //! }
    //! ```
    //! Here's an example Program, that takes 3 arguments, one of which can take a value:
    //! ```rust
    //! use revparse::{ArgState, Parser};
    //! let mut parser = Parser::new("parser");
    //! parser.add_argument(
    //!     "--start-process",                               // long name
    //!     Some("-s"),                                      // short name (optional)
    //!     "Start some Process, this is the help message!", // help message
    //!     Some("process"), // takes a value, in the help message this will be shown as --start-process=PROCESS
    //! );
    //! parser.add_argument("--reload", Some("-r"), "Reload the page", None); // no value is taken by this argument,
    //! parser.add_argument("--load", Some("-l"), "Load the page", None);
    //! parser.run();
    //! let start_process = match parser.get("--start-process") {
    //!     ArgState::False => "wasn't called".to_string(),
    //!     ArgState::True => panic!("Impossible!"),
    //!     ArgState::Value(s) => format!("was called with '{}' as an argument", s),
    //! };
    //! println!("\n--start-process {}", start_process);
    //! let reload = match parser.get("--reload") {
    //!     ArgState::False => "wasn't called".to_string(),
    //!     ArgState::True => "was called".to_string(),
    //!     ArgState::Value(_) => panic!("Impossible!"), // which is why this outcome here is impossible
    //! };
    //! println!("--reload {}", reload);
    //! let load = match parser.get("--load") {
    //!     ArgState::False => "wasn't called".to_string(),
    //!     ArgState::True => "was called".to_string(),
    //!     ArgState::Value(_) => panic!("Impossible!"),
    //! };
    //! println!("--load {}", load);
    //! ```
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
        for (i, s) in &self.args {
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
            "{}\n\n{}",
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
    fn val_missing(&self, arg: &str) {
        println!(
            "{}: option '{}' requires an argument\n{}\nTry '{} --help' for more information.",
            self.program_name,
            arg,
            self.usage.as_ref().unwrap(),
            self.program_name,
        );
    }
    /// Parses the arguments, and stores them in self or exits with the appropriate Error message.
    /// You have to run this function before using the .get() function.
    pub fn run(&mut self) {
        self.run_priv(env::args());
    }
    fn run_priv(&mut self, args: impl Iterator<Item = String>) {
        self.create_help();
        let mut next_is_val: Option<String> = None;
        self.parsed = Some(HashMap::new());
        let parsed = self.parsed.as_mut().unwrap();
        'outer: for e_arg in args.skip(1) {
            if next_is_val.is_some() {
                parsed.insert(next_is_val.unwrap(), Some(e_arg));
                next_is_val = None;
                continue 'outer;
            }
            if e_arg == "--help" || e_arg == "-h" {
                self.print_help();
                exit(0);
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
                                    exit(1);
                                }
                            }
                        }
                        self.arg_does_not_exist(&arg_name);
                        exit(1);
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
                        exit(1);
                    }
                }
            } else if e_arg.starts_with("-") {
                let mut rest_is_val: Option<String> = None;
                let mut value: Option<String> = None;
                'chars: for char in e_arg.chars().skip(1) {
                    if rest_is_val.is_some() {
                        if value.is_none() {
                            value = Some(char.to_string());
                        } else {
                            value.as_mut().unwrap().push(char);
                        }
                    } else {
                        for (p_arg, prop) in &self.args {
                            match prop.short_name {
                                Some(sp_arg) => {
                                    if format!("-{}", char) == sp_arg {
                                        if prop.take_value.is_some() {
                                            rest_is_val = Some(p_arg.to_string());
                                            continue 'chars;
                                        } else {
                                            parsed.insert(p_arg.to_string(), None);
                                            continue 'chars;
                                        }
                                    }
                                }
                                None => (),
                            }
                        }
                        self.arg_does_not_exist(&format!("-{}", char));
                        exit(1);
                    }
                }
                if rest_is_val.is_some() {
                    if value.is_none() {
                        next_is_val = Some(rest_is_val.unwrap());
                    } else {
                        parsed.insert(rest_is_val.unwrap(), value);
                    }
                }
            } else {
                self.arg_does_not_exist(&e_arg);
                exit(1);
            }
        }
        if next_is_val.is_some() {
            self.val_missing(next_is_val.as_ref().unwrap());
            exit(1);
        }
    }
    /// Function to get the results of the arguments. Returns an instance of ArgState.
    /// Example code:
    /// ```rust
    /// use revparse::{ArgState, Parser};
    /// let mut parser = Parser::new("your_program_name");
    /// parser.add_argument("--start-process", Some("-s"), "Start some process, this is the help message", Some("PROCESS"));
    /// parser.run();
    /// let result: ArgState = parser.get("--start-process");
    /// match result {
    ///     ArgState::True => panic!("Impossible"), // True will only be the case, if you didn't allow a value
    ///     ArgState::False => println!("Argument '--start-process' was not called"),
    ///     ArgState::Value(value) => println!("Argument '--start-process' was called with the value: '{value}'"),
    /// }
    /// ```
    pub fn get(&mut self, long_name: &'a str) -> ArgState {
        match self.parsed.as_mut().expect(ERROR_1).remove(long_name) {
            None => ArgState::False,
            Some(v) => match v {
                None => ArgState::True,
                Some(v) => ArgState::Value(v),
            },
        }
    }
    /// Function to create an instance of Parser, on which you call the .add_argument() function, as well as .get() and .run()
    /// Example code:
    /// ```rust
    /// use revparse::Parser;
    /// let mut parser = Parser::new("your_program_name");
    /// ```
    pub fn new(program_name: &'a str) -> Parser<'a> {
        Parser {
            args: Vec::new(),
            parsed: None,
            program_name,
            usage: None,
            help: None,
        }
    }
    /// To add arguments, you can use the .add_argument() function on a Parser instance.
    /// The function takes 4 Parameters apart from self.
    /// The First is the long name, that has to start with "--" and is required, not optional.
    /// The Second is an optional short name, of type Option<&str>. If it is set to None, there will be no short name for that argument, if you want a short name, like "-e" you will have to wrap it in Some() like this Some("-e"). Short names have to start with a '-' and only contain one other character.
    /// The Third option is the help message, that will be shown behind the corresponding option, when --help is called.
    /// The Fourth options is about wheter the argument can take values, or arguments like this:
    /// ```sh
    /// your_program_name --option-that-takes-a-value="This is the value"
    /// your_program_name --option-that-takes-a-value "This is the value"
    /// your_program_name -o"This is the value"
    /// your_program_name -o "This is the value"
    /// ```
    /// If you want this to be possible, you have to provide a name for the value to be shown in the help message wrapped in a Some().
    /// For example to add an argument "--start-process" that takes a value "PROCESS" you have to write the following:
    /// ```rust
    /// use revparse::Parser;
    /// let mut parser = Parser::new("your_program_name");
    /// parser.add_argument("--start-process", Some("-s"), "Start some process, this is the help message", Some("PROCESS"));
    /// ```
    /// You don't have to provide "PROCESS" in capital letters, since they will be capitalized automatically. This is what "PROCESS" is needed for:
    /// ```txt
    /// Usage: your_program_name [OPTION]...
    ///
    /// Options:
    ///   -s, --start-process=PROCESS  Start some process, this is the help message
    ///   ^-2 ^-1.parameter   ^-4.p.   ^-3.parameter
    /// ```
    ///
    pub fn add_argument(
        &mut self,
        long_name: &'a str,
        short_name: Option<&'a str>,
        help_msg: &'a str,
        take_value: Option<&'a str>,
    ) {
        self.args.push((
            long_name,
            ArgProp {
                short_name,
                help_msg,
                take_value,
            },
        ));
    }
}
