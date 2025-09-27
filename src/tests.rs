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
use basic::Revparse;
#[test]
fn val_long_vers() {
    let mut rvp = Revparse::custom_new(args(&["program_name", "--some-option", "some_val"]));
    let pos_args = rvp.get_pos_args();
    assert_eq!(pos_args.len(), 0);
    assert_eq!(rvp.some_option.unwrap(), String::from("some_val"));
}
#[test]
#[should_panic]
fn val_left_away() {
    Revparse::custom_new(args(&["n", "--some-option"]));
}
#[test]
#[should_panic]
fn too_many_pos() {
    Revparse::custom_new(args(&["n", "pos_arg", "pos_arg2"]));
}
#[test]
fn pos_args() {
    let mut rvp = Revparse::custom_new(args(&["n", "pos_arg"]));
    let pos_args = rvp.get_pos_args();
    assert_eq!(pos_args.len(), 1);
    assert_eq!(pos_args[0], "pos_arg");
}
#[test]
fn all_args() {
    let mut rvp = Revparse::custom_new(args(&[
        "n",
        "pos",
        "-osval",
        "--neither",
        "--no-short=abc"
    ]));
    let pos_args = rvp.get_pos_args();
    assert_eq!(pos_args.len(), 1);
    assert!(rvp.neither);
    assert!(rvp.option_noval);
    assert_eq!(rvp.some_option.unwrap(), "val");
    assert_eq!(rvp.no_short.unwrap(), "abc");
}
#[test]
#[should_panic]
fn short_arg_unrecognized() {
    Revparse::custom_new(args(&["n", "-z"]));
}

#[test]
#[should_panic]
fn arg_does_not_exit() {
    Revparse::custom_new(args(&["n", "--non-existant-arg"]));
}
revparse!{
    [some, 's', "help_message"];
    [PosInfinite => true];
    [ModName => rev];
}
#[test]
fn pos_infinite() {
    let mut rvp = rev::Revparse::custom_new(args(&["n", "1", "2", "3", "4", "5", "6", "last"]));
    let pos_args = rvp.get_pos_args();
    assert_eq!(pos_args.len(), 7);
    for i in 0..6 {
        assert_eq!(pos_args[i], format!("{}", i + 1))
    }
    assert_eq!(pos_args[6], "last");
}
#[test]
#[should_panic]
fn h_char() {
    Revparse::custom_new(args(&["n", "-oh"]));
}
revparse! {
    [PosMin => 2];
    [PosMax => 2];
    [ModName => pos_mod];
}
#[test]
#[should_panic]
fn max_pos() {
    pos_mod::Revparse::custom_new(args(&["n", "a", "b", "too many now"]));
}
#[test]
#[should_panic]
fn min_pos() {
    pos_mod::Revparse::custom_new(args(&["n", "only one positionl argument"]));
}
#[test]
fn test_minus_minus() {
    let mut rvp = pos_mod::Revparse::custom_new(args(&["n", "--", "--pos_arg", "--", "-another"]));
    let pos_args = rvp.get_pos_args();
    assert_eq!(pos_args.len(), 2);
    assert_eq!(pos_args[0], "--pos_arg");
    assert_eq!(pos_args[1], "-another");
}
revparse! {
    [ModName => no_pos];
}
#[test]
#[should_panic]
fn no_pos_allowed() {
    no_pos::Revparse::custom_new(args(&["n", "--"]));
}
#[test]
#[should_panic]
fn no_pos_allowed2() {
    no_pos::Revparse::custom_new(args(&["n", "pos"]));
}
revparse! {
    [r#fn, 'f', "help message"];
    [ModName => keyword];
}
#[test]
fn use_keyword_as_flag() {
    let args = keyword::Revparse::custom_new(args(&["n", "--fn"]));
    assert_eq!(args.r#fn, true);
}