use std::{collections::HashMap, env, mem};
#[cfg(test)]
mod tests;
const ERROR_1: &'static str = "The 'run' function has to be executed before this Function!!! Consult the Documentation for more... ERROR_CODE: '1'";
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
    pres_pos_args: Option<Vec<&'a str>>,
    max_pos_args_left: u16,
    pos_arg_help: Option<&'a str>,
    parsed_pos_args: Option<Vec<String>>,
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
    //! The .add_pos_arg() function can be used to add Positional Arguments (Arguments, that are passed without a flag, for example PATTERN in `grep <PATTERN>`)
    //! Usage:
    //! ```rust
    //! use revparse::Parser;
    //! let mut parser = Parser::new("your_program_name");
    //! parser.add_pos_arg("DIRECTORY"); // can be any name, if not in capital letters, it will be capitalized anyways.
    //! parser.add_pos_arg("FILE"); // you can add as many positional arguments, as you want.
    //! ```
    //!
    //! Parsed Positional Arguments can be requested using the `.get_pos_arg()` function
    //! The type of pos_args is `Option<Vec<String>>`.
    //! If there were no positional arguments given by the user, it will be None.
    //! All positional arguments given by the user, as far as allowed, will be pushed onto the Vector as a String.
    //!
    //! Usage:
    //! ```rust
    //! use revparse::{Parser, ArgState};
    //! let mut parser = Parser::new("your_program_name");
    //! parser.run();
    //! parser.add_pos_arg("DIRECTORY");
    //! let pos_args: Vec<String> = parser.get_pos_args();
    //! match pos_args.len() {
    //!     0 => println!("No positional argument was given"),
    //!     1 => println!("Arg was: {}", pos_args[0]),
    //!     _ => panic!("The Vectors length can't exceed the amount of times the add_pos_arg() function was called."),
    //! };
    //! let mut parser = Parser::new("test");
    //! parser.add_pos_arg("ARG1");
    //! parser.add_pos_arg("ARG2");
    //! parser.add_argument("--some-flag-that-takes-a-value", None, "Help msg", Some("VALUE"));
    //! // The `run_custom_args()` function allows you to provide the Program with custom Arguments, that aren't from the command line. It does the exact same thing as `run()`.
    //! parser.run_custom_args(Parser::args(&["target/debug/revparse", "POSITIONAL_ARGUMENT1", "--some-flag-that-takes-a-value", "value", "POSITIONAL_ARGUMENT2"]));
    //! let pos_args: Vec<String> = parser.get_pos_args();
    //! assert_eq!(pos_args[0], "POSITIONAL_ARGUMENT1");
    //! assert_eq!(pos_args[1], "POSITIONAL_ARGUMENT2");
    //! match parser.get("--some-flag-that-takes-a-value") {
    //!     ArgState::Value(val) => assert_eq!(val, "value"),
    //!     _ => panic!("Arguments were passed incorrectly!"),
    //! };
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
        if self.pos_arg_help.is_some() {
            self.help = Some(format!(
                "{}\n\nOptions:\n",
                self.pos_arg_help.unwrap().to_owned()
            ));
        } else {
            self.help = Some(String::from("\n\nOptions:\n"));
        }
        self.usage = Some(String::from(format!(
            "Usage: {} [OPTION]...",
            self.program_name
        )));
        if self.pres_pos_args.is_some() {
            for i in self.pres_pos_args.as_ref().unwrap() {
                self.usage
                    .as_mut()
                    .unwrap()
                    .push_str(&format!(" {}", i.to_uppercase()))
            }
        }
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
            "{}{}",
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
        self.run_custom_args(env::args());
    }
    /// ## Replaces the run() function, this function allows you to provide the program with custom Arguments other than std::env::args().
    pub fn run_custom_args(&mut self, args: impl Iterator<Item = String>) {
        self.create_help();
        let mut next_is_val: Option<String> = None;
        self.parsed = Some(HashMap::new());
        let parsed = self.parsed.as_mut().unwrap();
        'outer: for e_arg in args.skip(1) {
            if next_is_val.is_some() { parsed.insert(next_is_val.unwrap(), Some(e_arg));
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
                if self.pres_pos_args.is_some() {
                    if self.max_pos_args_left <= 0 {
                        self.arg_does_not_exist(&e_arg);
                        exit(1);
                    }
                    if self.parsed_pos_args.is_none() {
                        self.parsed_pos_args = Some(Vec::new());
                    }
                    self.parsed_pos_args.as_mut().unwrap().push(e_arg);
                    self.max_pos_args_left -= 1;
                } else {
                    self.arg_does_not_exist(&e_arg);
                    exit(1);
                }
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
            pres_pos_args: None,
            max_pos_args_left: 0,
            pos_arg_help: None,
            parsed_pos_args: None,
        }
    }
    /// # Help Message for Positional Arguments
    /// ## Optional
    /// ## Example Usage:
    /// ```rust
    /// use revparse::Parser;
    /// let mut parser = Parser::new("grep");
    /// parser.add_pos_arg("PATTERNS");
    /// parser.add_pos_arg("[FILE]...");
    /// // If you were to implement the help message of GNU grep:
    /// parser.pos_arg_help("Search for PATTERNS in each FILE.\nExample: grep -i 'hello world' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.");
    /// parser.run(); // if the user now passes --help, the pos_arg_help message will be printed under "Usage: ..."
    /// ```
    /// Which would look like this:
    /// ```txt
    /// Usage: grep [OPTION]... PATTERNS [FILE]...
    /// Search for PATTERNS in each FILE.
    /// Example: grep -i 'hello world' menu.h main.c
    /// PATTERNS can contain multiple patterns separated by newlines.
    /// ```
    pub fn pos_arg_help(&mut self, help_msg: &'a str) {
        self.pos_arg_help = Some(help_msg);
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
    /// # Adds Positional Arguments
    /// Usage:
    /// ```rust
    /// use revparse::Parser;
    /// let mut parser = Parser::new("your_program_name");
    /// parser.add_pos_arg("DIRECTORY"); // can be any name, if not in capital letters, it will be capitalized.
    /// parser.add_pos_arg("FILE"); // you can add as many positional arguments, as you want.
    /// parser.add_pos_arg("[FILE2]..."); // The "[]..." can be used to tell the user, that the argument is optional.
    /// parser.add_pos_arg("[MODE]..."); // The names are needed for the help message.
    /// ```
    pub fn add_pos_arg(&mut self, name: &'a str) {
        self.max_pos_args_left += 1;
        if self.pres_pos_args.is_none() {
            self.pres_pos_args = Some(Vec::new());
        }
        self.pres_pos_args.as_mut().unwrap().push(name);
    }
    /// ## Returns a Vector with all Positional arguments: `<Vec<String>`
    /// ```rust
    /// use revparse::Parser;
    /// let mut parser = Parser::new("your_program_name");
    /// parser.add_pos_arg("ARG");
    /// parser.run();
    /// let pos_args: Vec<String> = parser.get_pos_args();
    /// match pos_args.len() {
    ///     0 => println!("No positional argument was given"),
    ///     1 => println!("Arg was: {}", pos_args[0]),
    ///     _ => panic!("The Vectors length can't exceed the amount of times the add_pos_arg() function was called."),
    /// }
    /// ```
    pub fn get_pos_args(&mut self) -> Vec<String> {
        match mem::replace(&mut self.parsed_pos_args, None) {
            Some(vec) => vec,
            None => Vec::new(),
        }
    }
    pub fn args(args: &[&str]) -> impl Iterator<Item = String> {
        args.iter().map(|i| i.to_string())
    }
}
