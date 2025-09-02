# revparse
## Usage
First you have to create an instance of the Parser struct and provide the name of your Program, that will later be used for those cases:
```
your_program_name: unrecognized option '-a'
Usage: your_program_name [OPTION]...
Try 'your_program_name --help' for more information.
```
You can create an instance of Parser by calling the associated new() function with your programs name as an argument and assigning the returned Parser instance to a mutable variable (it has to be mutable!):
```rust
let mut parser = Parser::new("your_program_name");
```
To add arguments, you can use the .add_argument() function on parser.
The function takes 4 Parameters apart from self.

The First is the long name, that has to start with "--" and is required, not optional.

The Second is an optional short name, of type Option<&str>. If it is set to None, there will be no short name for that argument, if you want a short name, like "-e" you will have to wrap it in Some() like this Some("-e"). Short names have to start with a '-' and only contain one other character.

The Third option is the help message, that will be shown behind the corresponding option, when --help is called.

The Fourth options is about wheter the argument can take values, or arguments like this:
```
your_program_name --option-that-takes-a-value="This is the value"
your_program_name --option-that-takes-a-value "This is the value"
your_program_name -o"This is the value"
your_program_name -o "This is the value"
```
If you want this to be possible, you have to provide a name for the value to be shown in the help message wrapped in a Some().
For example to add an argument "--start-process" that takes a value "PROCESS" you have to write the following:
```rust
let mut parser = Parser::new("your_program_name");
parser.add_argument("--start-process", Some("-s"), "Start some process, this is the help message", Some("PROCESS"));
```
You don't have to provide "PROCESS" in capital letters, since they will be capitalized automatically. This is what "PROCESS" is needed for:
```
Usage: your_program_name [OPTION]...

Options:
  -s, --start-process=PROCESS  Start some process, this is the help message
  ^-1 ^-2.parameter   ^-4.p.   ^-3.parameter
```

To get the value of the arguments, you can use the .get() function defined on Parser. But before you can do that, you'll have to call .run():
```rust
let mut parser = Parser::new("your_program_name");
parser.add_argument("--start-process", Some("-s"), "Start some process, this is the help message", Some("PROCESS"));
parser.run();
```

Then you can call the .get() function on parser and provide the long name of your argument as a function parameter, which will return an enum called ArgState with three possible variants:

True
False
Value(String)

True will be returned, if the argument doesn't require a value to be inserted into it, as with --start-process="Value" and was called.
False will be returned, if the argument wasn't called, no matter wheter a value is needed or not.
Value(String) will be returned, if the argument needs a value, and was called with one. You are given ownership of the returned String.

You can best handle ArgState with a match expression like this:
```rust
let mut parser = Parser::new("your_program_name");
parser.add_argument("--start-process", Some("-s"), "Start some process, this is the help message", Some("PROCESS"));
parser.run();
let result: ArgState = parser.get("--start-process");
match result {
    ArgState::True => panic!("Impossible"), // True will only be the case, if you didn't allow a value
    ArgState::False => println!("Argument '--start-process' was not called"),
    ArgState::Value(value) => println!("Argument '--start-process' was called with the value: '{value}'"),
}
```
The .add_pos_arg() function can be used to add Positional Arguments (Arguments, that are passed without a flag, for example PATTERN in `grep <PATTERN>`)
Usage:
```rust
use revparse::Parser;
let mut parser = Parser::new("your_program_name");
parser.add_pos_arg("DIRECTORY"); // can be any name, if not in capital letters, it will be capitalized anyways.
parser.add_pos_arg("FILE"); // you can add as many positional arguments, as you want.
```

Parsed Positional Arguments can seen in the only public Structure field of Parser: pos_args

The type of pos_args is `Option<Vec<String>>`.
If there were no positional arguments given by the user, it will be None.
All positional arguments given by the user, as far as allowed, will be pushed onto the Vector as a String.

Usage:
```rust
use revparse::Parser;
let mut parser = Parser::new("your_program_name");
parser.add_pos_arg("DIRECTORY");
parser.run();
if parser.pos_args.len() != 0 {
    println!("The user entered following positional arguments: {:?}", vec);
} else {
    pritnln!("The user did not enter any positional arguments.");
}
```

Here's an example Program:
```rust
use revparse::{ArgState, Parser};
fn main() {
    let mut parser = Parser::new("parser");
    parser.add_argument(
        "--start-process",                               // long name
        Some("-s"),                                      // short name (optional)
        "Start some Process, this is the help message!", // help message
        Some("process"), // takes a value, in the help message this will be shown as --start-process=PROCESS
    );
    parser.add_argument("--reload", Some("-r"), "Reload the page", None); // no value is taken by this argument,
    parser.add_argument("--load", Some("-l"), "Load the page", None);
    parser.run();
    let start_process = match parser.get("--start-process") {
        ArgState::False => "wasn't called".to_string(),
        ArgState::True => panic!("Impossible!"),
        ArgState::Value(s) => format!("was called with '{}' as an argument", s),
    };
    println!("\n--start-process {}", start_process);
    let reload = match parser.get("--reload") {
        ArgState::False => "wasn't called".to_string(),
        ArgState::True => "was called".to_string(),
        ArgState::Value(_) => panic!("Impossible!"), // which is why this outcome here is impossible
    };
    println!("--reload {}", reload);
    let load = match parser.get("--load") {
        ArgState::False => "wasn't called".to_string(),
        ArgState::True => "was called".to_string(),
        ArgState::Value(_) => panic!("Impossible!"),
    };
    println!("--load {}", load);
}
```
