# revparse
## Usage
First you have to create a mutable variable, we'll call it parser
```rust
let mut parser = revparse::Parser::new("executable_name"); // for grep "executable_name" would be "grep"
```
There are two types of arguments the user can give

One that takes a value, and one that doesn't:
```txt
grep --version      // takes no value
grep --file=FILE    // takes a value
```
### In this example we will add an argument of the first type, that takes no value
```rust
let mut parser = revparse::Parser::new("grep");
parser.add_argument(
    "--version",
    Some("-V"),     // Short name (optional)
    "display version information and exit", // help message
    None,           // Take no value
);
```
#### How the argument will be shown in the help message:
```txt
  -V, --version             display version information and exit
```
### In this example we will add an argument of the second type, that takes a value
```rust
let mut parser = revparse::Parser::new("grep");
parser.add_argument(
    "--file",
    Some("-f"),     // Short name (optional)
    "take PATTERNS from FILE",  // help message
    Some("FILE"),   // Take a value called FILE to help the user understand what it is for
);
```
#### How the argument will be shown in the help message:
```txt
  -f, --file=FILE           take PATTERNS from FILE
```
### Find out if the user gave us an Option of the first type (That takes no value), or not
### (Using the 'get_noval' function)
```rust
let mut parser = revparse::Parser::new("grep");
parser.add_argument("--version", Some("-V"), "display version information and exit", None);
parser.run();   // Will parse the arguments the user entered
let version: bool = parser.get_noval("--version");
assert_eq!(version, false); // Since this is a doc-test, the flag will not have been given
```
### Get the value the user entered for the second type (That takes a value)
### (Using the 'get_val' function)
```rust
let mut parser = revparse::Parser::new("grep");
parser.add_argument("--file", Some("-f"), "take PATTERNS from FILE", Some("FILE"));
parser.run();
let file: Option<String> = parser.get_val("--file"); // DIFFERENT FUNCTION THAN ABOVE !!!

// The 'file' variable will be None, if the user didn't enter this flag, and Some(String) if he did
assert_eq!(file, None); // Since this is a doc-test, the flag will not have been given
```
### Examples
Since we want to simulate the user giving us flags (often called Options), we will use the run_custom_args() function instead of run()
```rust
let mut parser = revparse::Parser::new("grep");
parser.add_argument("--test", Some("-t"), "this is a test flag, that takes a value", Some("TEST_VALUE"));
parser.run_custom_args(&["program_name", "-t", "some_value"]); // --test will work just the same
let test: Option<String> = parser.get_val("--test");
assert_eq!(test.unwrap(), "some_value");
```
```rust
let mut parser = revparse::Parser::new("grep");
parser.add_argument("--test", Some("-t"), "this is a test flag, that takes a value", Some("TEST_VALUE"));
// This time the user doesn't give us an argument
parser.run_custom_args(&["program_name"]);
let test: Option<String> = parser.get_val("--test");
assert_eq!(test, None);     // Which is why the 'test' variable is None, and not Some(String)
```
```rust
let mut parser = revparse::Parser::new("grep");
parser.add_argument("--test", Some("-t"), "this is a test flag, that takes no value", None);
parser.run_custom_args(&["program_name", "-t"]);    // again, --test will work the same
let test: bool = parser.get_noval("--test");        // you can't use "-t" for this function
assert_eq!(test, true);
```
```rust
let mut parser = revparse::Parser::new("grep");
parser.add_argument("--test", Some("-t"), "this is a test flag, that takes no value", None);
parser.run_custom_args(&["program_name"]);      // this time the user gave us no arguments
let test: bool = parser.get_noval("--test");     
assert_eq!(test, false);                        // which is why the 'test' variable is false
```
## Positional Arguments
revparse supports the use of positional arguments, which are values passed without flags.

Like this:
```txt
program some_value
```
Instead of
```txt
program --value=some_value
```
### Usage
```rust
let mut parser = revparse::Parser::new("executable_name");
parser.add_pos_arg("ARGUMENT");
```
#### Help message
##### With a positional argument:
```txt
Usage: executable_name [OPTION]... ARGUMENT

Options:
  -h, --help                display this help text and exit
```
##### Without a positional argument:
```txt
Usage: executable_name [OPTION]...

Options:
  -h, --help                display this help text and exit
```
As you can see, the ARGUMENT after `[OPTION]...` vanished

### Here's how to force the user to enter at least 1 out of 2 positional arguments
```rust
let mut parser = revparse::Parser::new("executable_name");
parser.add_pos_arg("ARGUMENT1");    // This argument will be forced
parser.add_pos_arg("[ARGUMENT2]");  // This argument won't be forced
// Since we want the user to enter the first one, but don't care
// about whether he enters a second positional argument, we will pass 1 to this function
parser.min_pos_args(1);
// If you were to give 2 to that function, the user would have to enter 2 positional arguments
```
### Getting the value of positional arguments
```rust
// Example from above
let mut parser = revparse::Parser::new("executable_name");
parser.add_pos_arg("ARGUMENT1");
parser.add_pos_arg("[ARGUMENT2]");
parser.min_pos_args(1);
parser.run_custom_args(&["executable_name", "first_positional_argument", "--", "---second positional argument"]);
// The second positional argument starts with --- to demonstrate, that the user will have to type -- before such an argument
let pos_args: Vec<String> = parser.get_pos_args();
// parser.get_pos_args() returns a Vector with ALL positional arguments given
assert_eq!(pos_args.len(), 2); // The user entered both positional arguments, so the length is 2
assert_eq!(pos_args[0], "first_positional_argument");
assert_eq!(pos_args[1], "---second positional argument");
```
## Help message for positional arguments
Sometimes positional arguments need an explanation, like with grep:
```txt
Usage: grep [OPTION]... PATTERNS [FILE]...
Search for PATTERNS in each FILE.
Example: grep -i 'hello world' menu.h main.c
PATTERNS can contain multiple patterns separated by newlines.
```
To implement the above help message with this library you can use the 'pos_arg_help' function:
```rust
let mut parser = revparse::Parser::new("grep");
parser.add_pos_arg("PATTERNS");
parser.add_pos_arg("[FILE]...");
parser.min_pos_args(1);         // Force the user to enter a PATTERN
parser.pos_arg_help("Search for PATTERNS in each FILE.
Example: grep -i 'hello world' menu.h main.c
PATTERNS can contain multiple patterns separated by newlines.");
```