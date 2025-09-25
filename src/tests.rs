use super::*;
fn args(args: &[&str]) -> impl Iterator<Item = String> {
    args.iter().map(|i| i.to_string())
}
revparse! {
    [some_option, 's', "help message", "value"];
    [option_noval, 'o', "help message"];
    [no_short, "help message", "some"];
    [neither, "help message"];
    [ModName => basic];
    [Pos => "[some]"];
}
#[test]
fn basic_test() {
    let mut rvp = basic::Revparse::custom_new(args(&["program_name", "--some-option", "some_val"]));
    let pos_args = rvp.get_pos_args();
    assert_eq!(pos_args.len(), 0);
    assert_eq!(rvp.some_option.unwrap(), String::from("some_val"));
}
