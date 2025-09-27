# revparse
Compile time errors, fast parsing, easy usage.

## Usage
This argument parser works using the revparse! macro, into which you will write all the options:

`...` is just a placeholder.
```rust
use revparse::revparse;
revparse! {
    [...];
    [...];
    [...];
}
```
`revparse! {...}` should be written outside of any functions, as it creates a module called `revmod` by default.
You can change the name of the module using the [ModName](#modname) [setting](#settings).

## There's four different types of non positional arguments you can add:

### 1. long (--some-arg), short (-s), help message
```rust
[some_arg, 's', "help message"];
```
### 2. long (--no-short), help message
```rust
[no_short, "help message"];
```
### 3. long (--value=VALUE), short (-v VALUE / -vVALUE), help message, VALUE
`VALUE` is for making the user understand what value you want him to enter.
For example `FILE` would mean, the user should enter a filename.
```rust
[value, 'v', "help message", "VALUE"];
```
### 4. long (--takes-val-no-short=SOME), help message, SOME
```rust
[takes_val_no_short, "help message", "SOME"];
```

### All the above options together:
```rust
revparse! {
    [some_arg, 's', "help message"];
    [no_short, "help message"];
    [value, 'v', "help message", "VALUE"];
    [takes_val_no_short, "help message", "SOME"];
}
```
### How the help message would look like as of now:
```txt
Usage: program_name [OPTION]...

Options:
  -h, --help                display this help text and exit
  -s, --some-arg            help message
  --no-short                help message
  -v, --value=VALUE         help message
  --takes-val-no-short=SOME  help message
```
Of course "help message" isn't a very useful help message

As you can see in the help message, it says `program_name`, which probably isn't what you want.

You can change it using the [ExecName](#execname) [setting](#settings).

## Accessing the values

`new()` parses environmental args,
for tests you can use `custom_new()`, more about it [here](#custom-args-for-testing).
```rust
// creates module revmod
revparse! {
    [some_arg, 's', "help message"];
    [no_short, "help message"];
    [value, 'v', "help message", "VALUE"];
    [takes_val_no_short, "help message", "SOME"];
}
fn main() {
    let args = revmod::Revparse::new(); 
    println!("{:#?}", args);
}
```
This would print (if the user entered no arguments):
```rust
Revparse {
    some_arg: false,
    no_short: false,
    value: None,
    takes_val_no_short: None,
}
```
==The Arguments that take a value have the type `Option<String>`, those that don't have the type `bool`.==

## Custom args for testing
You can use the `custom_new()` function for testing your program with preset arguments.
`custom_new()` takes `impl Iterator<Item = String>` as a parameter.

So let's test the example above:
```rust
revparse! {
    [some_arg, 's', "help message"];
    [no_short, "help message"];
    [value, 'v', "help message", "VALUE"];
    [takes_val_no_short, "help message", "SOME"];
}
// helper function
fn iter_str(args: &[&str]) -> impl Iterator<Item = String> {
    args.iter().map(|i| i.to_string())
}
fn main() {
    let args = revmod::Revparse::custom_new(iter_str(&["exec", "--some-arg", "-vEXAMPLE_VALUE", "--takes-val-no-short", "some_value"]));
    assert_eq!(args.some_arg, true);    // was called
    assert_eq!(args.no_short, false);   // wasn't called
    assert_eq!(args.value.unwrap(), "EXAMPLE_VALUE"); // If -v had not been called, args.value would be `None` and the program would panic.
    assert_eq!(args.takes_val_no_short.unwrap(), "some_value");
    
    // So let's test it with different args
    let args = revmod::Revparse::custom_new(iter_str(&["exec", "-s", "--value=VAL", "--no-short"]));
    assert_eq!(args.some_arg, true); // was called with "-s"
    assert_eq!(args.no_short, true);
    assert_eq!(args.value.unwrap(), "VAL");
    assert_eq!(args.takes_val_no_short, None);  // is None, since it wasn't called
}
```
## Positional Arguments

Positional arguments are arguments that do not start with a "-" or "--".

If the user wants to give a positional argument, that does in fact start with a "-", he can write a "--" before the positional argument like this:
```bash
program_name -- "----positional argument"
```

If you don't know what positional arguments are, read [this](https://betterdev.blog/command-line-arguments-anatomy-explained/).

There are six [settings](#settings) for positional arguments.
1. [Pos](#pos)
2. [PosHelp](#poshelp)
3. [MinPos](#minpos)
4. [MaxPos](#maxpos)
5. [InfinitePos](#infinitepos)
6. [ModName](#modname)

To get the positional arguments, the user entered you can use the `get_pos_args()` function.
```rust
revparse! {
    [PosMax => 5];
    [PosMin => 1];
}
fn main() {
    let mut args = revmod::Revparse::new();
    let pos_args = args.get_pos_args();
    let len = pos_args.len();
    assert!(len <= 5 && len >= 1); // User has to enter beween 1 and 5 positional arguments.
}
```

### Implementing this for GNU grep
```txt
Usage: grep [OPTION]... PATTERNS [FILE]...
Search for PATTERNS in each FILE.
Example: grep -i 'hello world' menu.h main.c
PATTERNS can contain multiple patterns separated by newlines.
```
To implement this, you would have to use these settings:
```rust
[PosHelp => "Search for PATTERNS in each FILE.\nExample: grep -i 'hello world' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines."];
[Pos => "PATTERNS"];
[Pos => "[FILE]..."];
[ExecName => "grep"];
[InfinitePos => true]; // grep has no limit for the amount of files you can enter.
[MinPos => 1]; // and forces you to enter a Pattern
```

## Settings
The Settings syntax is as follows
```rust
[SettingName => ...];
```
The following Settings exist:

\[[ExecName](#execname) => \<string literal\>\];

\[[Pos](#pos) => \<string literal\>\];

\[[PosHelp](#poshelp) => \<string literal\>\];

\[[MinPos](#min) => u64\];

\[[MaxPos](#maxpos) => u64\];

\[[InfinitePos](#infinitepos) => bool\];

\[[ModName](#modname) => \<identifier\>\];

### ExecName
The name of the executable. Needed for the help message.

Default: `program_name`

To change it to `revparse` for example:
```rust
[ExecName => "revparse"];
```

### Pos
Setting can be given multiple times.
Each `Pos` setting will be displayed in the "Usage message".

This:
```rust
[Pos => "SOME"];
[Pos => "ANOTHER"];
```
would be displayed like this:
```txt
Usage: program_name [OPTION]... SOME ANOTHER
```
[More](#positional-arguments)

### PosHelp
Help message for positional arguments.
For example
```rust
[PosHelp => "POS HELP MESSAGE"];
```
would be shown in the help message as:
```txt
Usage: program_name [OPTION]...
POS HELP MESSAGE

Options:
...
```
In case you wonder for what this is, [here is an example](#implementing-this-for-gnu-grep).

### MinPos
The minimum amount of [Positional arguments](#positional-arguments) the user has to enter.

Default is `0`.

### MaxPos
The maximum amount of [Positional arguments](#positional-arguments) the user has to enter.

Default is the amount of times
```rust
[Pos => "SOME"];
```
was used.
This default can be overwritten with either [\[MaxPos => ...\];](#maxpos) or [\[InfinitePos => ...\];](#infinitepos).

### InfinitePos
If this is set to `true`,
```rust
[InfinitePos => true];
```
there will be no limit, on how much [Positional arguments](#positional-arguments) the user can enter.

### ModName
Name of the module created by the `revparse!` macro.
Default is `revmod`.
If you want to change it to `example`:
```rust
[ModName => example];
```
`example` can't be a [keyword](https://doc.rust-lang.org/reference/keywords.html#keywords) and should not be written in quotes.