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
    min_pos_args: u16,
    max_pos_args_left: u16,
    pos_arg_help: Option<&'a str>,
    parsed_pos_args: Option<Vec<String>>,
}
impl<'a> Parser<'a> {
    //! The Parser struct is the Heart of revparse.
    //! Here is a brief explanation.
    //! # revparse
    //! ## Usage
    //! ```rust
    //! // Import the Parser struct, and ArgState enum
    //! use revparse::{ArgState, Parser};
    //! // Create an instance of Parser
    //! let mut parser: Parser = Parser::new("your_program_name"); // your_program_name is needed for the help message
    //! // Add argument
    //! parser.add_argument(
    //!     "--argument",                  // Long Name, not optional
    //!     Some("-a"),                    // Optional short name
    //!     "Argument does this and that", // Help message for that specific argument
    //!     // If this is Some(), then the argument will require a value to be passed to it
    //!     Some("VALUE"),                 // like this: your_program_name -a "value"
    //! );
    //!
    //! // Call this function between adding the arguments and getting them
    //! parser.run(); // Will take the arguments passed to the program
    //! // Alternatively you can use run_custom_args() for testing
    //! // parser.run_custom_args(Parser::args(&["your_program_name", "-a", "value"]));
    //!
    //! let argument: ArgState = parser.get("--argument");
    //! match argument {
    //!     ArgState::False => println!("--argument wasn't called"),
    //!     ArgState::Value(val) => println!("--argument was called with the value: {}", val),
    //!     ArgState::True => panic!("Impossible, ArgState::True will only be returned, if the last argument to parser.add_argument() is None."),
    //! }
    //! ```
    //! # Positional Arguments
    //! Positional Arguments are Values passed without flags.
    //! ## Usage
    //! ```rust
    //! use revparse::Parser;
    //! let mut parser: Parser = Parser::new("grep");
    //! // This would store the first argument, that doesn't start with '-' AND isn't after a flag, that takes a value.
    //! parser.add_pos_arg(
    //!     "PATTERN",
    //!     false,   // if false, then the Positional argument is not required to be given by the user
    //! );
    //! parser.run();
    //! let pos_args: Vec<String> = parser.get_pos_args();
    //! if pos_args.len() != 0 {
    //!     // So in this case 'grep smth' would give you the String "smth" in pos_args[0].
    //!     // The string can't start with '-', unless the user types -- before it:
    //!     // grep -- "-string"
    //!     println!("The first positional argument was: {}", pos_args[0]);
    //! }
    //! ```
    //! # Examples
    //! ### Example Program with flag '-a', that takes a value and flag '-b', that doesn't
    //! ```rust
    //! use revparse::{ArgState, Parser};
    //! let mut parser: Parser = Parser::new("your_program_name");
    //! parser.add_argument("--arg-a", Some("-a"), "Takes a value", Some("VAL_NAME"));
    //! parser.add_argument("--arg-b", Some("-b"), "Does not take a value", None);
    //! // Normally you would call .run(), but in this example we will call .run_custom_args() instead, to test it.
    //! parser.run_custom_args(Parser::args(&[
    //!     "your_program_name", // Program name will be ignored
    //!     "-avalue",           // "-a" "value" is valid too
    //!     "-b",
    //! ]));
    //! let value_passed_to_a: String = match parser.get("--arg-a") {
    //!     ArgState::Value(s) => s,
    //!     _ => panic!("Parsing Error!"),
    //! };
    //! assert_eq!(value_passed_to_a, "value");
    //! if let ArgState::True = parser.get("--arg-b") {
    //!     // true, as arg was called
    //! } else {
    //!     panic!("Parsing Error!")
    //! }
    //! ```
    //! #### Help Message:
    //! ```txt
    //! Usage: your_program_name [OPTION]...
    //!
    //! Options:
    //!   -a, --arg-a=VAL_NAME      Takes a value
    //!   -b, --arg-b               Does not take a value
    //! ```
    //! ### Previous Example Program with 2 Positional Arguments
    //! ```rust
    //! use revparse::{ArgState, Parser};
    //! let mut parser: Parser = Parser::new("your_program_name");
    //! parser.add_argument("--arg-a", Some("-a"), "Takes a value", Some("VAL_NAME"));
    //! parser.add_argument("--arg-b", Some("-b"), "Does not take a value", None);
    //! parser.add_pos_arg("EXAMPLE", true); // User is forced to provide that positional argument
    //! parser.add_pos_arg("[ANOTHER]...", false); // User doesn't have to provide that positional argument
    //! // You can see the help message format below
    //! parser.pos_arg_help("Help Message Shown under 'Usage:', EXAMPLE can be used to ... etc\nCan contain new line chars.");
    //! // Normally you would call .run(), but in this example we will call .run_custom_args() instead, to test it.
    //! parser.run_custom_args(Parser::args(&[
    //!     "your_program_name",// Program name will be ignored
    //!     "--arg-a=value",    // "-a" "value" is valid too
    //!     "--",               // means the next arg will be a positional argument
    //!     "-pos arg that starts with -", // Valid, because it is the next argument after --
    //!     "-b",
    //!     "This is a positional Argument, because -b does not take a value",
    //! ]));
    //! // From previos code
    //! let value_passed_to_a: String = match parser.get("--arg-a") {
    //!     ArgState::Value(s) => s,
    //!     _ => panic!("Parsing Error!"),
    //! };
    //! assert_eq!(value_passed_to_a, "value");
    //! if let ArgState::True = parser.get("--arg-b") {
    //!     // true, as arg was called
    //! } else {
    //!     panic!("Parsing Error!")
    //! }
    //!
    //! // Positional Arguments
    //! let pos_args: Vec<String> = parser.get_pos_args();
    //! assert_eq!(pos_args.len(), 2); // Length is 2, as two positional Arguments were provided.
    //! assert_eq!(pos_args[0], "-pos arg that starts with -");
    //! assert_eq!(pos_args[1], "This is a positional Argument, because -b does not take a value");
    //! ```
    //! #### Help Message:
    //! ```txt
    //! Usage: your_program_name [OPTION]... EXAMPLE [ANOTHER]...
    //! Help Message Shown under 'Usage:', EXAMPLE can be used to ... etc
    //! Can contain new line chars.
    //!
    //! Options:
    //!   -a, --arg-a=VAL_NAME      Takes a value
    //!   -b, --arg-b               Does not take a value
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
                "\n{}\n\nOptions:\n",
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
        let help = self.help.as_mut().unwrap();
        help.push_str("  -h, --help                display this help text and exit\n");
        for (i, s) in &self.args {
            let mut length: i8; //28 chars between help_msg and the beginning of the line
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
                length -= s.take_value.as_ref().unwrap().len() as i8 + 1;
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
        let mut next_is_pos: bool = false;
        self.parsed = Some(HashMap::new());
        let parsed = self.parsed.as_mut().unwrap();
        'outer: for e_arg in args.skip(1) {
            if next_is_val.is_some() {
                parsed.insert(next_is_val.unwrap(), Some(e_arg));
                next_is_val = None;
                continue 'outer;
            }
            if next_is_pos {
                next_is_pos = false;
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
                    if self.min_pos_args > 0 {
                        self.min_pos_args -= 1;
                    }
                } else {
                    self.arg_does_not_exist(&e_arg);
                    exit(1);
                }
                continue 'outer;
            }
            if e_arg == "--help" || e_arg == "-h" {
                self.print_help();
                exit(0);
            } else if e_arg == "--" {
                next_is_pos = true;
                continue 'outer;
            } else if e_arg.starts_with("--") {
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
                    if self.min_pos_args > 0 {
                        self.min_pos_args -= 1;
                    }
                } else {
                    self.arg_does_not_exist(&e_arg);
                    exit(1);
                }
            }
        }
        if next_is_val.is_some() {
            self.val_missing(next_is_val.as_ref().unwrap());
            exit(1);
        } else if self.min_pos_args != 0 {
            self.print_usage();
            exit(1);
        }
    }
    fn print_usage(&self) {
        println!(
            "{}\nTry '{} --help' for more information.",
            self.usage.as_ref().unwrap(),
            self.program_name
        );
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
            min_pos_args: 0,
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
    /// parser.add_pos_arg("PATTERNS", false);
    /// parser.add_pos_arg("[FILE]...", false);
    /// // If you were to implement the help message of GNU grep:
    /// parser.pos_arg_help("Search for PATTERNS in each FILE.\nExample: grep 'hello world' file.txt");
    /// // Disclaimer: This is not the official help message of GNU grep, but merely an example
    /// parser.run(); // if the user now passes --help, the pos_arg_help message will be printed under "Usage: ..."
    /// ```
    /// Which would look like this:
    /// ```txt
    /// Usage: grep [OPTION]... PATTERNS [FILE]...
    /// Search for PATTERNS in each FILE.
    /// Example: grep 'hello world' file.txt
    /// ```
    pub fn pos_arg_help(&mut self, help_msg: &'a str) {
        self.pos_arg_help = Some(help_msg);
    }
    /// # Adds Arguments
    /// ## Usage
    /// ```rust
    /// use revparse::Parser;
    /// let mut parser: Parser = Parser::new("your_program_name");
    /// // call add_argument() on a mutable Parser instance
    /// parser.add_argument(
    ///     "--takes-no-value", // long name
    ///     Some("-n"),         // *optional* short name
    ///     "Takes no value",   // Help message
    ///     None,               // Take no value if None
    /// );
    /// parser.add_argument(
    ///     "--takes-value",    // Same as before
    ///     None,               // This time without a short name
    ///     "Takes a value, eg. --takes-value=\"value\"",
    ///     Some("VALUE"),      // Brief keyword about the value for the help message
    /// );
    /// ```
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
    /// parser.add_pos_arg("DIRECTORY", true); // can be any name, if not in capital letters, it will be capitalized.
    /// parser.add_pos_arg("FILE", true); // you can add as many positional arguments, as you want.
    /// parser.add_pos_arg("REQUIRED", true); // if the second argument to the function is true, the user is forced to give that positional argument
    /// parser.add_pos_arg("[FILE2]...", false); // The "[]..." can be used to tell the user, that the argument is optional.
    /// parser.add_pos_arg("[MODE]...", true); // The names are needed for the help message.
    /// ```
    pub fn add_pos_arg(&mut self, name: &'a str, required: bool) {
        if required {
            self.min_pos_args += 1;
        }
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
    /// parser.add_pos_arg("ARG", false);
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
