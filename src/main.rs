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
        ArgState::False => "wasn't called".to_string(),
        ArgState::True => panic!("Impossible!"),
        ArgState::Value(s) => format!("is {}",s),
    };
    println!("--start-process {}", start_process);
    let reload = match argparse.get("--reload") {
        ArgState::False => "wasn't called".to_string(),
        ArgState::True => "was called".to_string(),
        ArgState::Value(_) => panic!("Impossible!"),
    };
    println!("--reload {}", reload);
    let load = match argparse.get("--load") {
        ArgState::False => "wasn't called".to_string(),
        ArgState::True => "was called".to_string(),
        ArgState::Value(_) => panic!("Impossible!"),
    };
    println!("--load {}", load);
}
