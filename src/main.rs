use revparse::{ArgState, Parser};
fn main() {
    let mut parser = Parser::new("parser");
    parser.add_argument(
        "--start-process",                               // long name
        Some("-s"),                                      // short name (optional)
        "Start some Process, this is the help message!", // help message
        Some("process"), // takes a value, in the help message this will be shown as --start-process=PROCESS
    );
    parser.add_argument("--reload", Some("-r"), "Reload the page", None);
    parser.add_argument("--load", Some("-l"), "Load the page", None);
    parser.add_pos_arg("DIRECTORY");
    parser.add_pos_arg("[FILE]...");
    parser.run();
    let start_process = match parser.get("--start-process") {
        ArgState::False => "wasn't called".to_string(),
        ArgState::True => panic!("Impossible!"),
        ArgState::Value(s) => format!("was called with '{}' as an argument", s),
    };
    println!("\n--start-process {}", start_process);
    let reload = match parser.get("--reload") {
        ArgState::False => "wasn't called".to_string(),
        ArgState::True => "was called".to_string(),
        ArgState::Value(_) => panic!("Impossible!"),
    };
    println!("--reload {}", reload);
    let load = match parser.get("--load") {
        ArgState::False => "wasn't called".to_string(),
        ArgState::True => "was called".to_string(),
        ArgState::Value(_) => panic!("Impossible!"),
    };
    println!("--load {}", load);
    match parser.pos_args {
        Some(vec) => {
            let mut c = 0;
            for i in vec {
                c += 1;
                println!("Positional arg {}: '{}'", c, i);
            }
        }
        None => println!("No positional arguments given."),
    }
}
