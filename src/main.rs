use argparse::{ArgState, Parser};
fn main() {
    let mut argparse = Parser::new("argparse");
    argparse.add_argument(
        "--start-process",
        Some("-s"),
        "Start some Process, this is the help message!",
        Some("process"),
    );
    argparse.add_argument("--reload", Some("-r"), "Reload the page", None);
    argparse.add_argument("--load", Some("-l"), "Load the page", None);
    argparse.run();
    let start_process = match argparse.get("--start-process") {
        ArgState::False => "Arg wasn't called".to_string(),
        ArgState::True => "Arg was called".to_string(),
        ArgState::Value(s) => s,
    };
    println!("{}", start_process)
}
