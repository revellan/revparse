use super::*;
fn base() -> Parser<'static> {
    let mut parser = Parser::new("test");
    parser.add_argument("--test", Some("-t"), "This is just a test", Some("test"));
    parser.add_argument("--noval", Some("-n"), "Testmsg", None);
    parser
}
fn pos_args() -> Parser<'static> {
    let mut parser = Parser::new("test");
    parser.add_pos_arg("TEST", false);
    parser.add_pos_arg("TEST2", false);
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
    let result: ArgState = parser.get("--start-process"); // if there is a typo in --start-process, the program will panic at runtime
    match result {
        ArgState::True => panic!("Impossible"), // True will only be the case, if you didn't allow a value
        ArgState::False => (),
        ArgState::Value(_) => panic!("Impossible"),
    }
}
#[test]
fn take_value_short_name() {
    let mut parser = base();
    parser.run_custom_args(Parser::args(&["program_name", "-tvalue"]));
    if let ArgState::Value(s) = parser.get("--test") {
        assert_eq!("value", s);
    } else {
        panic!("Wrong Return Type");
    }
}
#[test]
fn take_value_short_name_distant() {
    let mut parser = base();
    parser.run_custom_args(Parser::args(&["n", "-t", "value"]));
    if let ArgState::Value(s) = parser.get("--test") {
        assert_eq!("value", s);
    } else {
        panic!("Wrong Return Type");
    }
}
#[test]
fn take_value_long_name_eq_sign() {
    let mut parser = base();
    parser.run_custom_args(Parser::args(&["n", "--test=value"]));
    if let ArgState::Value(s) = parser.get("--test") {
        assert_eq!("value", s);
    } else {
        panic!("Wrong Return Type");
    }
}
#[test]
fn take_value_long_name_distant() {
    let mut parser = base();
    parser.run_custom_args(Parser::args(&["n", "--test", "value"]));
    if let ArgState::Value(s) = parser.get("--test") {
        assert_eq!("value", s);
    } else {
        panic!("Wrong Return Type");
    }
}
#[test]
#[should_panic]
fn take_value_not_given() {
    let mut parser = base();
    parser.run_custom_args(Parser::args(&["n", "--test"]));
}

#[test]
#[should_panic]
fn take_value_not_given_short_name() {
    let mut parser = base();
    parser.run_custom_args(Parser::args(&["n", "-t"]));
}
#[test]
#[should_panic]
fn take_value_not_given_short_name_2() {
    let mut parser = base();
    parser.run_custom_args(Parser::args(&["n", "-nt"]));
}
#[test]
fn positional_arguments() {
    let mut parser = pos_args();
    parser.run_custom_args(Parser::args(&["n", "PARG1", "-abc", "PARG2"]));
    let vec = parser.get_pos_args();
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], "PARG1");
    assert_eq!(vec[1], "PARG2");
    let mut parser = pos_args();
    parser.run_custom_args(Parser::args(&[
        "n",
        "--aletter",
        "--value",
        "value",
        "--",
        "-PARG1",
        "-bc",
    ]));
    let vec = parser.get_pos_args();
    assert_eq!(vec.len(), 1);
    assert_eq!(vec[0], "-PARG1");
    let value = match parser.get("--value") {
        ArgState::Value(v) => v,
        _ => panic!("Failed to parse value."),
    };
    assert_eq!(value, "value");
}