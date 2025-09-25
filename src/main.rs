use revparse::revparse;
revparse! {
    [some_arg, 's', "help message", "value"];
    [ModName => module];
    [Pos => "pos_arg"];
    [PosMin => 0];
    [PosMax => 10];
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
}
