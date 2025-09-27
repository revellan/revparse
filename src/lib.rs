/// # Usage
/// This argument parser works using the revparse! macro, into which you will write all the options:
/// 
/// `...` is just a placeholder.
/// ```no_compile
/// use revparse::revparse;
/// revparse! {
///     [...];
///     [...];
///     [...];
/// }
/// ```
/// `revparse! {...}` should be written outside of any functions, as it creates a module called `revmod` by default.
/// You can change the name of the module using the [ModName setting](#modname).
/// 
/// ## There's four different types of non positional arguments you can add:
/// 
/// ### 1. long (--some-arg), short (-s), help message
/// ```no_compile
/// [some_arg, 's', "help message"];
/// ```
/// ### 2. long (--no-short), help message
/// ```no_compile
/// [no_short, "help message"];
/// ```
/// ### 3. long (--value=VALUE), short (-v VALUE / -vVALUE), help message, VALUE
/// `VALUE` is for making the user understand what value you want him to enter.
/// For example `FILE` would mean, the user should enter a filename.
/// ```no_compile
/// [value, 'v', "help message", "VALUE"];
/// ```
/// ### 4. long (--takes-val-no-short=SOME), help message, SOME
/// ```no_compile
/// [takes_val_no_short, "help message", "SOME"];
/// ```
/// 
/// ### All the above options together:
/// ```rust
/// # use revparse_derive::revparse;
/// revparse! {
///     [some_arg, 's', "help message"];
///     [no_short, "help message"];
///     [value, 'v', "help message", "VALUE"];
///     [takes_val_no_short, "help message", "SOME"];
/// }
/// ```
/// ### How the help message would look like as of now:
/// ```txt
/// Usage: program_name [OPTION]...
/// 
/// Options:
///   -h, --help                display this help text and exit
///   -s, --some-arg            help message
///   --no-short                help message
///   -v, --value=VALUE         help message
///   --takes-val-no-short=SOME  help message
/// ```
/// Of course "help message" isn't a very useful help message.
/// 
/// As you can see in the help message, it says `program_name`, which probably isn't what you want.
/// 
/// You can change it using the [ExecName setting](#execname).
/// 
/// ## Accessing the values
/// 
/// `new()` parses environmental args,
/// for tests you can use `custom_new()`, more about it [here](#custom-args-for-testing).
/// ```rust
/// # use revparse_derive::revparse;
/// // creates module revmod
/// revparse! {
///     [some_arg, 's', "help message"];
///     [no_short, "help message"];
///     [value, 'v', "help message", "VALUE"];
///     [takes_val_no_short, "help message", "SOME"];
/// }
/// fn main() {
///     let args = revmod::Revparse::new(); 
///     // args.some_arg => bool
///     // args.no_short => bool
///     // args.value => Option<String>
///     // args.takes_val_no_short => Option<String>
///     println!("{:#?}", args);
/// }
/// ```
/// This would print (if the user entered no arguments):
/// ```no_compile
/// Revparse {
///     some_arg: false,
///     no_short: false,
///     value: None,
///     takes_val_no_short: None,
/// }
/// ```
/// <mark>The Arguments that take a value have the type:</mark>
/// ```no_compile
/// Option<String>
/// ```
/// If the flag was not given, the value will be `None`.
/// 
/// <mark>Those that don't take a value have the type:</mark>
/// ```no_compile
/// bool
/// ```
/// If the flag was not given, the value will be `false`, if it was, it will be `true`.
/// 
/// ## Custom args for testing
/// You can use the `custom_new()` function for testing your program with preset arguments.
/// `custom_new()` takes `impl Iterator<Item = String>` as a parameter.
/// 
/// So let's test the example above:
/// ```rust
/// # use revparse_derive::revparse;
/// revparse! {
///     [some_arg, 's', "help message"];
///     [no_short, "help message"];
///     [value, 'v', "help message", "VALUE"];
///     [takes_val_no_short, "help message", "SOME"];
/// }
/// // helper function
/// fn iter_str(args: &[&str]) -> impl Iterator<Item = String> {
///     args.iter().map(|i| i.to_string())
/// }
/// fn main() {
///     let args = revmod::Revparse::custom_new(iter_str(&["exec", "--some-arg", "-vEXAMPLE_VALUE", "--takes-val-no-short", "some_value"]));
///     assert_eq!(args.some_arg, true);    // was called
///     assert_eq!(args.no_short, false);   // wasn't called
///     assert_eq!(args.value.unwrap(), "EXAMPLE_VALUE"); // If -v had not been called, args.value would be `None` and the program would panic.
///     assert_eq!(args.takes_val_no_short.unwrap(), "some_value");
///     
///     // So let's test it with different args
///     let args = revmod::Revparse::custom_new(iter_str(&["exec", "-s", "--value=VAL", "--no-short"]));
///     assert_eq!(args.some_arg, true); // was called with "-s"
///     assert_eq!(args.no_short, true);
///     assert_eq!(args.value.unwrap(), "VAL");
///     assert_eq!(args.takes_val_no_short, None);  // is None, since it wasn't called
/// }
/// ```
/// ## Positional Arguments
/// 
/// positional arguments are arguments that do not start with a "-" or "--".
/// 
/// If the user wants to give a positional argument, that does in fact start with a "-", he can write a "--" before the positional argument like this:
/// ```bash
/// program_name -- "----positional argument"
/// ```
/// 
/// If you don't know what positional arguments are, read [this](https://betterdev.blog/command-line-arguments-anatomy-explained/).
/// 
/// There are six [settings](#settings) for positional arguments.
/// 1. [Pos](#pos)
/// 2. [PosHelp](#poshelp)
/// 3. [PosMin](#posmin)
/// 4. [PosMax](#posmax)
/// 5. [PosInfinite](#posinfinite)
/// 6. [ModName](#modname)
/// 
/// ### Get Positional Arguments
/// To get the positional arguments, the user entered you can use the `get_pos_args()` function.
/// ```no_run
/// # use revparse_derive::revparse;
/// revparse! {
///     [PosMax => 5];
///     [PosMin => 1];
/// }
/// fn main() {
///     let mut args = revmod::Revparse::new();
///     let pos_args = args.get_pos_args();
///     let len = pos_args.len();
///     assert!(len <= 5 && len >= 1); // User has to enter beween 1 and 5 positional arguments.
/// }
/// ```
/// 
/// ### Implementing this for GNU grep
/// ```txt
/// Usage: grep [OPTION]... PATTERNS [FILE]...
/// Search for PATTERNS in each FILE.
/// Example: grep -i 'hello world' menu.h main.c
/// PATTERNS can contain multiple patterns separated by newlines.
/// ```
/// To implement this, you would have to use these settings:
/// ```no_compile
/// [PosHelp => "Search for PATTERNS in each FILE.\nExample: grep -i 'hello world' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines."];
/// [Pos => "PATTERNS"];
/// [Pos => "[FILE]..."];
/// [ExecName => "grep"];
/// [PosInfinite => true]; // grep has no limit for the amount of files you can enter.
/// [PosMin => 1]; // and forces you to enter a Pattern
/// ```
/// 
/// ## Settings
/// The Settings syntax is as follows
/// ```no_compile
/// [SettingName => ...];
/// ```
/// The following Settings exist:
/// 
/// \[[ExecName](#execname) => \<string literal\>\];
/// 
/// \[[Pos](#pos) => \<string literal\>\];
/// 
/// \[[PosHelp](#poshelp) => \<string literal\>\];
/// 
/// \[[PosMin](#posmin) => u64\];
/// 
/// \[[PosMax](#posmax) => u64\];
/// 
/// \[[PosInfinite](#posinfinite) => bool\];
/// 
/// \[[ModName](#modname) => \<identifier\>\];
/// 
/// ### ExecName
/// The name of the executable. Needed for the help message.
/// 
/// Default: `program_name`
/// 
/// To change it to `revparse` for example:
/// ```no_compile
/// [ExecName => "revparse"];
/// ```
/// 
/// ### Pos
/// Setting can be given multiple times.
/// Each `Pos` setting will be displayed in the "Usage message".
/// 
/// This:
/// ```no_compile
/// [Pos => "SOME"];
/// [Pos => "ANOTHER"];
/// ```
/// would be displayed like this:
/// ```txt
/// Usage: program_name [OPTION]... SOME ANOTHER
/// ```
/// and would raise the default of [PosMax](#posmax) to `2`, as [\[Pos => ...\];](#pos) was given twice.
/// 
/// [More](#positional-arguments)
/// 
/// ### PosHelp
/// Help message for positional arguments.
/// For example
/// ```no_compile
/// [PosHelp => "POS HELP MESSAGE"];
/// ```
/// would be shown in the help message as:
/// ```txt
/// Usage: program_name [OPTION]...
/// POS HELP MESSAGE
/// 
/// Options:
/// ...
/// ```
/// In case you wonder for what this is, [here is an example](#implementing-this-for-gnu-grep).
/// 
/// ### PosMin
/// The minimum amount of [positional arguments](#positional-arguments) the user has to enter.
/// 
/// Default is `0`.
/// 
/// To force the user to enter `1` [positional argument](#positional-arguments):
/// ```no_compile
/// [PosMin => 1];
/// ```
/// 
/// ### PosMax
/// The maximum amount of [positional arguments](#positional-arguments) the user has to enter.
/// 
/// Default is the amount of times
/// ```no_compile
/// [Pos => "SOME"];
/// ```
/// was used.
/// 
/// To change it to `5`, which would mean, that the user can't enter more than `5` [positional arguments](#positional-arguments):
/// ```no_compile
/// [PosMax => 5];
/// ```
/// This default can be overwritten with either [\[PosMax => ...\];](#posmax) or [\[PosInfinite => ...\];](#posinfinite).
/// 
/// ### PosInfinite
/// If this is set to `true`,
/// ```no_compile
/// [PosInfinite => true];
/// ```
/// there will be no limit, on how much [positional arguments](#positional-arguments) the user can enter.
/// 
/// Default is `false`.
/// 
/// ### ModName
/// Name of the module created by the `revparse!` macro.
/// Default is `revmod`.
/// If you want to change it to `example`:
/// ```no_compile
/// [ModName => example];
/// ```
/// `example` can't be a [keyword](https://doc.rust-lang.org/reference/keywords.html#keywords) and should not be written in quotes.
pub use revparse_derive::revparse;
#[cfg(test)]
mod tests;