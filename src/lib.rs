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
/// The Parser Struct is the heart of revparse
/// Here's a brief explanation
/// # revparse
/// ## Usage
/// First you have to create a mutable variable, we'll call it parser
/// ```rust
/// let mut parser = revparse::Parser::new("executable_name"); // for grep "executable_name" would be "grep"
/// ```
/// There are two types of arguments the user can give
///
/// One that takes a value, and one that doesn't:
/// ```txt
/// grep --version      // takes no value
/// grep --file=FILE    // takes a value
/// ```
/// ### In this example we will add an argument of the first type, that takes no value
/// ```rust
/// let mut parser = revparse::Parser::new("grep");
/// parser.add_argument(
///     "--version",
///     Some("-V"),     // Short name (optional)
///     "display version information and exit", // Official GNU grep help message for --version
///     None,           // Take no value
/// );
/// ```
/// #### How the argument will be shown in the help message:
/// ```txt
///   -V, --version             display version information and exit
/// ```
/// ### In this example we will add an argument of the second type, that takes a value
/// ```rust
/// let mut parser = revparse::Parser::new("grep");
/// parser.add_argument(
///     "--file",
///     Some("-f"),     // Short name (optional)
///     "take PATTERNS from FILE",  // from GNU grep
///     Some("FILE"),   // Take a value called FILE to help the user understand what it is for
/// );
/// ```
/// #### How the argument will be shown in the help message:
/// ```txt
///   -f, --file=FILE           take PATTERNS from FILE
/// ```
/// ### Find out if the user gave us an Option of the first type (That takes no value), or not
/// ### (Using the 'get_noval' function)
/// ```rust
/// let mut parser = revparse::Parser::new("grep");
/// parser.add_argument("--version", Some("-V"), "display version information and exit", None);
/// parser.run();   // Will parse the arguments the user entered
/// let version: bool = parser.get_noval("--version");
/// assert_eq!(version, false); // Since this is a doc-test, the flag will not have been given
/// ```
/// ### Get the value the user entered for the second type (That takes a value)
/// ### (Using the 'get_val' function)
/// ```rust
/// let mut parser = revparse::Parser::new("grep");
/// parser.add_argument("--file", Some("-f"), "take PATTERNS from FILE", Some("FILE"));
/// parser.run();
/// let file: Option<String> = parser.get_val("--file"); // DIFFERENT FUNCTION THAN ABOVE !!!
///
/// // The 'file' variable will be None, if the user didn't enter this flag, and Some(String) if he did
/// assert_eq!(file, None); // Since this is a doc-test, the flag will not have been given
/// ```
/// ### Examples
/// Since we want to simulate the user giving us flags (often called Options), we will use the run_custom_args() function instead of run()
/// ```rust
/// let mut parser = revparse::Parser::new("grep");
/// parser.add_argument("--test", Some("-t"), "this is a test flag, that takes a value", Some("TEST_VALUE"));
/// parser.run_custom_args(&["program_name", "-t", "some_value"]); // --test will work just the same
/// let test: Option<String> = parser.get_val("--test");
/// assert_eq!(test.unwrap(), "some_value");
/// ```
/// ```rust
/// let mut parser = revparse::Parser::new("grep");
/// parser.add_argument("--test", Some("-t"), "this is a test flag, that takes a value", Some("TEST_VALUE"));
/// // This time the user doesn't give us an argument
/// parser.run_custom_args(&["program_name"]);
/// let test: Option<String> = parser.get_val("--test");
/// assert_eq!(test, None);     // Which is why the 'test' variable is None, and not Some(String)
/// ```
/// ```rust
/// let mut parser = revparse::Parser::new("grep");
/// parser.add_argument("--test", Some("-t"), "this is a test flag, that takes no value", None);
/// parser.run_custom_args(&["program_name", "-t"]);    // again, --test will work the same
/// let test: bool = parser.get_noval("--test");        // you can't use "-t" for this function
/// assert_eq!(test, true);
/// ```
/// ```rust
/// let mut parser = revparse::Parser::new("grep");
/// parser.add_argument("--test", Some("-t"), "this is a test flag, that takes no value", None);
/// parser.run_custom_args(&["program_name"]);      // this time the user gave us no arguments
/// let test: bool = parser.get_noval("--test");     
/// assert_eq!(test, false);                        // which is why the 'test' variable is false
/// ```
/// ## Positional Arguments
/// revparse supports the use of positional arguments, which are values passed without flags.
///
/// Like this:
/// ```txt
/// program some_value
/// ```
/// Instead of
/// ```txt
/// program --value=some_value
/// ```
/// ### Usage
/// ```rust
/// let mut parser = revparse::Parser::new("executable_name");
/// parser.add_pos_arg("ARGUMENT");
/// ```
/// #### Help message
/// ##### With a positional argument:
/// ```txt
/// Usage: executable_name [OPTION]... ARGUMENT
///
/// Options:
///   -h, --help                display this help text and exit
/// ```
/// ##### Without a positional argument:
/// ```txt
/// Usage: executable_name [OPTION]...
///
/// Options:
///   -h, --help                display this help text and exit
/// ```
/// As you can see, the ARGUMENT after `[OPTION]...` vanished
///
/// ### Here's how to force the user to enter at least 1 out of 2 positional arguments
/// ```rust
/// let mut parser = revparse::Parser::new("executable_name");
/// parser.add_pos_arg("ARGUMENT1");    // This argument will be forced
/// parser.add_pos_arg("[ARGUMENT2]");  // This argument won't be forced
/// // Since we want the user to enter the first one, but don't care
/// // about whether he enters a second positional argument, we will pass 1 to this function
/// parser.min_pos_args(1);
/// // If you were to give 2 to that function, the user would have to enter 2 positional arguments
/// ```
/// ### Getting the value of positional arguments
/// ```rust
/// // Example from above
/// let mut parser = revparse::Parser::new("executable_name");
/// parser.add_pos_arg("ARGUMENT1");
/// parser.add_pos_arg("[ARGUMENT2]");
/// parser.min_pos_args(1);
/// parser.run_custom_args(&["executable_name", "first_positional_argument", "--", "---second positional argument"]);
/// // The second positional argument starts with --- to demonstrate, that the user will have to type -- before such an argument
/// let pos_args: Vec<String> = parser.get_pos_args();
/// // parser.get_pos_args() returns a Vector with ALL positional arguments given
/// assert_eq!(pos_args.len(), 2); // The user entered both positional arguments, so the length is 2
/// assert_eq!(pos_args[0], "first_positional_argument");
/// assert_eq!(pos_args[1], "---second positional argument");
/// ```
/// ## Help message for positional arguments
/// Sometimes positional arguments need an explanation, like with grep:
/// ```txt
/// Usage: grep [OPTION]... PATTERNS [FILE]...
/// Search for PATTERNS in each FILE.
/// Example: grep -i 'hello world' menu.h main.c
/// PATTERNS can contain multiple patterns separated by newlines.
/// ```
/// To implement the above help message with this library you can use the 'pos_arg_help' function:
/// ```rust
/// let mut parser = revparse::Parser::new("grep");
/// parser.add_pos_arg("PATTERNS");
/// parser.add_pos_arg("[FILE]...");
/// parser.min_pos_args(1);         // Force the user to enter a PATTERN
/// parser.pos_arg_help("Search for PATTERNS in each FILE.
/// Example: grep -i 'hello world' menu.h main.c
/// PATTERNS can contain multiple patterns separated by newlines.");
/// ```
impl<'a> Parser<'a> {
    fn arg_does_not_exist(&self, arg: &str) {
        if arg == "--help" || arg == "-h" {
            self.no_val_allowed(arg);
        } else {
            eprintln!(
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
    /// # Print the help message
    /// Of course the help message will be printed automatically if the user enters -h | --help,
    /// but in case you want to do it manually I made this function public
    /// ## Usage
    /// ```rust
    /// let mut parser = revparse::Parser::new("executable_name");
    /// parser.add_argument("--test", Some("-t"), "test argument to demonstrate print_help", None);
    /// parser.run(); // This will create the help message
    /// parser.print_help(); // This will print it.
    /// ```
    /// ### What will be printed:
    /// ```txt
    /// Usage: executable_name [OPTION]...
    ///
    /// Options:
    ///   -h, --help                display this help text and exit
    ///   -t, --test                test argument to demonstrate print_help
    /// ```
    pub fn print_help(&self) {
        print!(
            "{}{}",
            self.usage.as_ref().unwrap(),
            self.help.as_ref().unwrap()
        );
    }
    fn no_val_allowed(&self, arg: &str) {
        eprintln!(
            "{}: option '{}' doesn't allow an argument\n{}\nTry '{} --help' for more information.",
            self.program_name,
            arg,
            self.usage.as_ref().unwrap(),
            self.program_name,
        );
    }
    fn val_missing(&self, arg: &str) {
        eprintln!(
            "{}: option '{}' requires an argument\n{}\nTry '{} --help' for more information.",
            self.program_name,
            arg,
            self.usage.as_ref().unwrap(),
            self.program_name,
        );
    }
    /// # Parse the arguments given by the user
    /// ## Usage
    /// ```rust
    /// let mut parser = revparse::Parser::new("executable_name");
    /// // Add arguments, etc...
    /// parser.run(); // Then call this function
    /// // Then get the arguments
    /// ```
    pub fn run(&mut self) {
        self.run_internal(env::args());
    }
    fn run_internal(&mut self, args: impl Iterator<Item = String>) {
        self.create_help();
        let mut next_is_val: Option<String> = None;
        let mut next_is_pos: bool = false;
        self.parsed = Some(HashMap::new());
        'outer: for e_arg in args.skip(1) {
            let parsed = self.parsed.as_mut().unwrap();
            if next_is_val.is_some() {
                parsed.insert(next_is_val.unwrap(), Some(e_arg));
                next_is_val = None;
                continue 'outer;
            }
            if next_is_pos {
                next_is_pos = false;
                self.internal_add_pos_arg(e_arg);
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
                self.internal_add_pos_arg(e_arg);
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
    /// # Print the usage message manually
    /// This is the message, that will automatically be printed, if the user enters a wrong argument, etc...
    /// ## grep usage message:
    /// ```txt
    /// Usage: grep [OPTION]... PATTERNS [FILE]...
    /// Search for PATTERNS in each FILE.
    /// Example: grep -i 'hello world' menu.h main.c
    /// PATTERNS can contain multiple patterns separated by newlines.
    /// ```
    /// ## How to implement it with revparse:
    /// ```rust
    /// let mut parser = revparse::Parser::new("grep");
    /// parser.add_pos_arg("PATTERNS");
    /// parser.add_pos_arg("[FILE]...");
    ///
    /// parser.pos_arg_help("Search for PATTERNS in each FILE.
    /// Example: grep -i 'hello world' menu.h main.c
    /// PATTERNS can contain multiple patterns separated by newlines.");
    /// parser.run();   // To create the usage message
    ///
    /// parser.print_usage();   // prints the message above
    /// ```
    pub fn print_usage(&self) {
        eprintln!(
            "{}\nTry '{} --help' for more information.",
            self.usage.as_ref().unwrap(),
            self.program_name
        );
    }
    /// ### Get the value the user entered for the second type (That takes a value)
    /// ```rust
    /// let mut parser = revparse::Parser::new("grep");
    /// parser.add_argument("--file", Some("-f"), "take PATTERNS from FILE", Some("FILE"));
    /// parser.run();
    /// let file: Option<String> = parser.get_val("--file");
    ///
    /// // The 'file' variable will be None, if the user didn't enter this flag, and Some(String) if he did
    /// assert_eq!(file, None); // Since this is a doc-test, the flag will not have been given
    /// ```
    /// ### Examples
    /// Since we want to simulate the user giving us flags (often called Options), we will use the run_custom_args() function instead of run()
    /// ```rust
    /// let mut parser = revparse::Parser::new("grep");
    /// parser.add_argument("--test", Some("-t"), "this is a test flag, that takes a value", Some("TEST_VALUE"));
    /// parser.run_custom_args(&["program_name", "-t", "some_value"]); // --test will work just the same
    /// let test: Option<String> = parser.get_val("--test");
    /// assert_eq!(test.unwrap(), "some_value");
    /// ```
    /// ```rust
    /// let mut parser = revparse::Parser::new("grep");
    /// parser.add_argument("--test", Some("-t"), "this is a test flag, that takes a value", Some("TEST_VALUE"));
    /// // This time the user doesn't give us an argument
    /// parser.run_custom_args(&["program_name"]);
    /// let test: Option<String> = parser.get_val("--test");
    /// assert_eq!(test, None);     // Which is why the 'test' variable is None, and not Some(String)
    /// ```
    pub fn get_val(&mut self, long_name: &'a str) -> Option<String> {
        match self.parsed.as_mut().expect(ERROR_1).remove(long_name) {
            None => None,
            Some(v) => match v {
                None => panic!(
                    "Option '{}' doesn't take a value, use the get_noval() function instead!!!",
                    long_name
                ),
                Some(v) => Some(v),
            },
        }
    }
    /// ### Find out if the user gave us an Option of the first type (That takes no value), or not
    /// ```rust
    /// let mut parser = revparse::Parser::new("grep");
    /// parser.add_argument("--version", Some("-V"), "display version information and exit", None);
    /// parser.run();   // Will parse the arguments the user entered
    /// let version: bool = parser.get_noval("--version");
    /// assert_eq!(version, false); // Since this is a doc-test, the flag will not have been given
    /// ```
    /// ### Examples
    /// ```rust
    /// let mut parser = revparse::Parser::new("grep");
    /// parser.add_argument("--test", Some("-t"), "this is a test flag, that takes no value", None);
    /// parser.run_custom_args(&["program_name", "-t"]);    // again, --test will work the same
    /// let test: bool = parser.get_noval("--test");        // you can't use "-t" for this function
    /// assert_eq!(test, true);
    /// ```
    /// ```rust
    /// let mut parser = revparse::Parser::new("grep");
    /// parser.add_argument("--test", Some("-t"), "this is a test flag, that takes no value", None);
    /// parser.run_custom_args(&["program_name"]);      // this time the user gave us no arguments
    /// let test: bool = parser.get_noval("--test");     
    /// assert_eq!(test, false);                        // which is why the 'test' variable is false
    /// ```
    pub fn get_noval(&mut self, long_name: &'a str) -> bool {
        match self.parsed.as_mut().expect(ERROR_1).remove(long_name) {
            None => false,
            Some(v) => match v {
                Some(_) => panic!(
                    "Option '{}' takes a value, use the get_val() function instead!!!",
                    long_name
                ),
                None => true,
            },
        }
    }
    /// # Create a new Parser struct
    /// Takes the name of the executable of your program as a parameter
    ///
    /// It is needed for the help message
    /// ```rust
    /// let mut parser = revparse::Parser::new("executable_name");
    /// // Add arguments, etc...
    /// parser.run();
    /// // Get arguments
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
    /// ## Help message for positional arguments
    /// Sometimes positional arguments need an explanation, like with grep:
    /// ```txt
    /// Usage: grep [OPTION]... PATTERNS [FILE]...
    /// Search for PATTERNS in each FILE.
    /// Example: grep -i 'hello world' menu.h main.c
    /// PATTERNS can contain multiple patterns separated by newlines.
    /// ```
    /// To implement the above help message with this library you can use the 'pos_arg_help' function:
    /// ```rust
    /// let mut parser = revparse::Parser::new("grep");
    /// parser.add_pos_arg("PATTERNS");
    /// parser.add_pos_arg("[FILE]...");
    /// parser.min_pos_args(1);         // Force the user to enter a PATTERN
    /// parser.pos_arg_help("Search for PATTERNS in each FILE.
    /// Example: grep -i 'hello world' menu.h main.c
    /// PATTERNS can contain multiple patterns separated by newlines.");
    /// ```
    pub fn pos_arg_help(&mut self, help_msg: &'a str) {
        self.pos_arg_help = Some(help_msg);
    }
    /// # Add argument
    /// There are two types of flag arguments the user can give
    ///
    /// One that takes a value, and one that doesn't:
    /// ```txt
    /// grep --version      // takes no value
    /// grep --file=FILE    // takes a value
    /// ```
    /// ### In this example we will add an argument of the first type, that takes no value
    /// ```rust
    /// let mut parser = revparse::Parser::new("grep");
    /// parser.add_argument(
    ///     "--version",
    ///     Some("-V"),     // Short name (optional)
    ///     "display version information and exit", // Official GNU grep help message for --version
    ///     None,           // Take no value
    /// );
    /// ```
    /// #### How the argument will be shown in the help message:
    /// ```txt
    ///   -V, --version             display version information and exit
    /// ```
    /// ### In this example we will add an argument of the second type, that takes a value
    /// ```rust
    /// let mut parser = revparse::Parser::new("grep");
    /// parser.add_argument(
    ///     "--file",
    ///     Some("-f"),     // Short name (optional)
    ///     "take PATTERNS from FILE",  // from GNU grep
    ///     Some("FILE"),   // Take a value called FILE to help the user understand what it is for
    /// );
    /// ```
    /// #### How the argument will be shown in the help message:
    /// ```txt
    ///   -f, --file=FILE           take PATTERNS from FILE
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
    fn internal_add_pos_arg(&mut self, e_arg: String) {
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
    /// ### Usage
    /// ```rust
    /// let mut parser = revparse::Parser::new("executable_name");
    /// parser.add_pos_arg("ARGUMENT");
    /// ```
    pub fn add_pos_arg(&mut self, name: &'a str) {
        self.max_pos_args_left += 1;
        if self.pres_pos_args.is_none() {
            self.pres_pos_args = Some(Vec::new());
        }
        self.pres_pos_args.as_mut().unwrap().push(name);
    }
    /// # Returns a Vector with all given positional arguments
    /// ### Usage
    /// ```rust
    /// let mut parser = revparse::Parser::new("executable_name");
    /// parser.add_pos_arg("ARGUMENT");
    /// parser.run();   // Parse arguments given by the user
    /// let pos_args: Vec<String> = parser.get_pos_args();
    /// if pos_args.len() == 1 {
    ///     println!("The user entered one positional argument: '{}", pos_args[0]);
    /// }
    /// ```
    pub fn get_pos_args(&mut self) -> Vec<String> {
        match mem::replace(&mut self.parsed_pos_args, None) {
            Some(vec) => vec,
            None => Vec::new(),
        }
    }
    /// # Simulate the users arguments
    /// ### Usage
    /// ```rust
    /// let mut parser = revparse::Parser::new("executable_name");
    /// parser.add_argument("--test", Some("-t"), "test argument", None);
    /// parser.run_custom_args(&[
    ///     "executable_name", // first argument is the program name
    ///     "--test"
    /// ]);
    /// let test_res = parser.get_noval("--test");
    /// assert!(test_res);
    /// ```
    pub fn run_custom_args(&mut self, args: &[&str]) {
        self.run_internal(args.iter().map(|i| i.to_string()));
    }
    /// # Force the user to enter at least amount of positional arguments, specified with this function
    /// ### Usage
    /// ```rust
    /// let mut parser = revparse::Parser::new("executable_name");
    /// parser.add_pos_arg("ARGUMENT");
    /// parser.min_pos_args(1); // now the user has to enter 'ARGUMENT'
    /// ```
    pub fn min_pos_args(&mut self, arg_amount: u16) {
        self.min_pos_args = arg_amount;
    }
}
