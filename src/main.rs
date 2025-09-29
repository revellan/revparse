use revparse::revparse;
revparse! {
    [some_arg, 's', "help message", "value"];
    [another, 'a', "help message of --another"];
    [ModName => module];
    [Pos => "pos_arg"];
    [PosMax => 10];
    [ExecName => "revparse"];
    [PosInfinite => true];
}
#[test]
fn test_main() {
    main();
}
fn main() {
    let mut rvp = module::Revparse::new();
    let pos_args = rvp.get_pos_args();
    for i in pos_args.iter().enumerate() {
        println!("Pos_arg {}: '{}'", i.0, i.1);
    }
    match rvp.some_arg {
        Some(val) => println!("--some-arg was called with: '{}'", val),
        None => println!("--some-arg was not called"),
    }
    if rvp.another {
        println!("--another was called");
    } else {
        println!("--another was NOT called");
    }
}
