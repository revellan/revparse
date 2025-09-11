use super::*;
fn base() -> Parser<'static> {
    let mut parser = Parser::new("test");
    parser.add_argument("--test", Some("-t"), "This is just a test", Some("test"));
    parser.add_argument("--noval", Some("-n"), "Testmsg", None);
    parser
}
fn pos_args() -> Parser<'static> {
    let mut parser = Parser::new("test");
    parser.add_pos_arg("TEST");
    parser.add_pos_arg("TEST2");
    parser.min_pos_args(1);
    parser.add_argument("--aletter", Some("-a"), "help msg", None);
    parser.add_argument("--bletter", Some("-b"), "help msg", None);
    parser.add_argument("--cletter", Some("-c"), "help msg", None);
    parser.add_argument("--value", Some("-v"), "help msg", Some("[VALUE]..."));
    parser
}
#[test]
fn basic_usage() {
    let mut parser = Parser::new("your_program_name");
    parser.add_argument(
        "--start-process",
        Some("-s"),
        "Start some process, this is the help message",
        Some("PROCESS"),
    );
    parser.run();
    let result: Option<String> = parser.get_val("--start-process"); // if there is a typo in --start-process, the program will panic at runtime;
    assert_eq!(result, None)
}
#[test]
fn take_value_short_name() {
    let mut parser = base();
    parser.run_custom_args(&["program_name", "-tvalue"]);
    let value = parser.get_val("--test");
    assert_eq!(value.unwrap(), "value");
}
#[test]
fn take_value_short_name_distant() {
    let mut parser = base();
    parser.run_custom_args(&["n", "-t", "value"]);
    let value = parser.get_val("--test");
    assert_eq!(value.unwrap(), "value");
}
#[test]
fn take_value_long_name_eq_sign() {
    let mut parser = base();
    parser.run_custom_args(&["n", "--test=value"]);
    let value = parser.get_val("--test");
    assert_eq!(value.unwrap(), "value");
}
#[test]
fn take_value_long_name_distant() {
    let mut parser = base();
    parser.run_custom_args(&["n", "--test", "value"]);
    let value = parser.get_val("--test");
    assert_eq!(value.unwrap(), "value");
}
#[test]
#[should_panic]
fn take_value_not_given() {
    let mut parser = base();
    parser.run_custom_args(&["n", "--test"]);
}

#[test]
#[should_panic]
fn take_value_not_given_short_name() {
    let mut parser = base();
    parser.run_custom_args(&["n", "-t"]);
}
#[test]
#[should_panic]
fn take_value_not_given_short_name_2() {
    let mut parser = base();
    parser.run_custom_args(&["n", "-nt"]);
}
#[test]
fn positional_arguments() {
    let mut parser = pos_args();
    parser.run_custom_args(&["n", "PARG1", "-abc", "PARG2"]);
    let vec = parser.get_pos_args();
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], "PARG1");
    assert_eq!(vec[1], "PARG2");
    let mut parser = pos_args();
    parser.run_custom_args(&["n", "--aletter", "--value", "value", "--", "-PARG1", "-bc"]);
    let vec = parser.get_pos_args();
    assert_eq!(vec.len(), 1);
    assert_eq!(vec[0], "-PARG1");
    let value = parser.get_val("--value");
    assert_eq!(value.unwrap(), "value");
}
#[test]
#[should_panic]
fn required_pos_args_not_given() {
    let mut parser = pos_args();
    parser.run_custom_args(&["n"]);
}
#[test]
#[should_panic]
fn wrong_function_to_get_args() {
    let mut parser = base();
    parser.run_custom_args(&["n", "--test", "value"]);
    parser.get_noval("--test");
}
