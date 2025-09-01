use super::*;
fn base() -> Parser<'static> {
    let mut parser = Parser::new("test");
    parser.add_argument("--test", Some("-t"), "This is just a test", Some("test"));
    parser.add_argument("--noval", Some("-n"), "Testmsg", None);
    parser
}
fn args(args: &[&str]) -> impl Iterator<Item = String> {
    args.iter().map(|i| i.to_string())
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
    parser.run_priv(args(&["program_name", "-tvalue"]));
    if let ArgState::Value(s) = parser.get("--test") {
        assert_eq!("value", s);
    } else {
        panic!("Wrong Return Type");
    }
}
#[test]
fn take_value_short_name_distant() {
    let mut parser = base();
    parser.run_priv(args(&["n", "-t", "value"]));
    if let ArgState::Value(s) = parser.get("--test") {
        assert_eq!("value", s);
    } else {
        panic!("Wrong Return Type");
    }
}
#[test]
fn take_value_long_name_eq_sign() {
    let mut parser = base();
    parser.run_priv(args(&["n", "--test=value"]));
    if let ArgState::Value(s) = parser.get("--test") {
        assert_eq!("value", s);
    } else {
        panic!("Wrong Return Type");
    }
}
#[test]
fn take_value_long_name_distant() {
    let mut parser = base();
    parser.run_priv(args(&["n", "--test", "value"]));
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
    parser.run_priv(args(&["n", "--test"]));
}

#[test]
#[should_panic]
fn take_value_not_given_short_name() {
    let mut parser = base();
    parser.run_priv(args(&["n", "-t"]));
}
#[test]
#[should_panic]
fn take_value_not_given_short_name_2() {
    let mut parser = base();
    parser.run_priv(args(&["n", "-nt"]));
}