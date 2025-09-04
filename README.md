# revparse
## Usage
```rust
// Import the Parser struct, and ArgState enum
use revparse::{ArgState, Parser};
// Create an instance of Parser
let mut parser: Parser = Parser::new("your_program_name"); // your_program_name is needed for the help message
// Add argument
parser.add_argument(
    "--argument",                  // Long Name, not optional
    Some("-a"),                    // Optional short name
    "Argument does this and that", // Help message for that specific argument
    // If this is Some(), then the argument will require a value to be passed to it
    Some("VALUE"),                 // like this: your_program_name -a "value"
);

// Call this function between adding the arguments and getting them
parser.run(); // Will take the arguments passed to the program
// Alternatively you can use run_custom_args() for testing
// parser.run_custom_args(Parser::args(&["your_program_name", "-a", "value"]));

let argument: ArgState = parser.get("--argument");
match argument {
    ArgState::False => println!("--argument wasn't called"),
    ArgState::Value(val) => println!("--argument was called with the value: {}", val),
    ArgState::True => panic!("Impossible, ArgState::True will only be returned, if the last argument to parser.add_argument() is None."),
}
```
# Positional Arguments
Positional Arguments are Values passed without flags.
## Usage
```rust
use revparse::Parser;
let mut parser: Parser = Parser::new("grep");
// This would store the first argument, that doesn't start with '-' AND isn't after a flag, that takes a value.
parser.add_pos_arg(
    "PATTERN",
    false,   // Positional argument is not required
);
parser.run();
let pos_args: Vec<String> = parser.get_pos_args();
if pos_args.len() != 0 {
    // So in this case 'grep smth' would give you the String "smth" in pos_args[0].
    // The string can't start with '-', unless the user types -- before it:
    // grep -- "-string"
    println!("The first positional argument was: {}", pos_args[0]);
}
```
# Examples
### Example Program with flag '-a', that takes a value and flag '-b', that doesn't
```rust
use revparse::{ArgState, Parser};
let mut parser: Parser = Parser::new("your_program_name");
parser.add_argument("--arg-a", Some("-a"), "Takes a value", Some("VAL_NAME"));
parser.add_argument("--arg-b", Some("-b"), "Does not take a value", None);
// Normally you would call .run(), but in this example we will call .run_custom_args() instead, to test it.
parser.run_custom_args(Parser::args(&[
    "your_program_name", // Program name will be ignored
    "-avalue",           // "-a" "value" is valid too
    "-b",
]));
let value_passed_to_a: String = match parser.get("--arg-a") {
    ArgState::Value(s) => s,
    _ => panic!("Parsing Error!"),
};
assert_eq!(value_passed_to_a, "value");
if let ArgState::True = parser.get("--arg-b") {
    // true, as arg was called
} else {
    panic!("Parsing Error!")
}
```
#### Help Message:
```txt
Usage: your_program_name [OPTION]...

Options:
  -a, --arg-a=VAL_NAME      Takes a value
  -b, --arg-b               Does not take a value
```
### Previous Example Program with 2 Positional Arguments
```rust
use revparse::{ArgState, Parser};
let mut parser: Parser = Parser::new("your_program_name");
parser.add_argument("--arg-a", Some("-a"), "Takes a value", Some("VAL_NAME"));
parser.add_argument("--arg-b", Some("-b"), "Does not take a value", None);
parser.add_pos_arg("EXAMPLE", true); // If not provided, the program will complain
parser.add_pos_arg("[ANOTHER]...", false); // The program will not complain
// You can see the help message format below
parser.pos_arg_help("Help Message Shown under 'Usage:', EXAMPLE can be used to ... etc\nCan contain new line chars.");
// Normally you would call .run(), but in this example we will call .run_custom_args() instead, to test it.
parser.run_custom_args(Parser::args(&[
    "your_program_name",// Program name will be ignored
    "--arg-a=value",    // "-a" "value" is valid too
    "--",               // means the next arg will be a positional argument
    "-pos arg that starts with -", // Valid, because it is the next argument after --
    "-b",
    "This is a positional Argument, because -b does not take a value",
]));
// From previos code
let value_passed_to_a: String = match parser.get("--arg-a") {
    ArgState::Value(s) => s,
    _ => panic!("Parsing Error!"),
};
assert_eq!(value_passed_to_a, "value");
if let ArgState::True = parser.get("--arg-b") {
    // true, as arg was called
} else {
    panic!("Parsing Error!")
}

// Positional Arguments
let pos_args: Vec<String> = parser.get_pos_args();
assert_eq!(pos_args.len(), 2); // Length is 2, as two positional Arguments were provided.
assert_eq!(pos_args[0], "-pos arg that starts with -");
assert_eq!(pos_args[1], "This is a positional Argument, because -b does not take a value");
```
#### Help Message:
```txt
Usage: your_program_name [OPTION]... EXAMPLE [ANOTHER]...
Help Message Shown under 'Usage:', EXAMPLE can be used to ... etc
Can contain new line chars.

Options:
  -a, --arg-a=VAL_NAME      Takes a value
  -b, --arg-b               Does not take a value
```