use argparse::{ArgState, Parser};
fn main() {
    let mut argparse = Parser::new("Program_name");
    argparse.add_argument(
        "--start-process",
        Some("-s"),
        "Start some Process, this is the help message!",
        Some("process"),
        true,
    );
    argparse.add_argument("--reload", Some("-r"), "Reload the page", None, false);
    argparse.add_argument("--load", Some("-l"), "Load the page", None, false);
    argparse.run();
    let load: bool = match argparse.get("--load") {
        ArgState::False => false,
        ArgState::True => true,
        ArgState::Value(_) => panic!("Impossible"),
    };
    println!("{load}");
}
