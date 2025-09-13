use revparse::Parser;
fn main() {
    let mut parser = Parser::new("parser");
    parser.add_argument(
        "--start-process",                               // long name
        Some("-s"),                                      // short name (optional)
        "start some Process, this is the help message!", // help message
        Some("process"), // takes a value, in the help message this will be shown as --start-process=PROCESS
    );
    parser.add_argument("--reload", Some("-r"), "reload the page", None);
    parser.add_argument("--load", Some("-l"), "load the page", None);
    parser.add_pos_arg("DIRECTORY");
    parser.add_pos_arg("[FILE]...");
    parser.min_pos_args(1);
    parser.allow_infinite_pos_args();
    parser.run();
    let start_process = match parser.get_val("--start-process") {
        None => "wasn't called".to_string(),
        Some(s) => format!("was called with '{}' as an argument", s),
    };
    println!("\n--start-process {}", start_process);
    let reload = match parser.get_noval("--reload") {
        false => "wasn't called".to_string(),
        true => "was called".to_string(),
    };
    println!("--reload {}", reload);
    let load = match parser.get_noval("--load") {
        false => "wasn't called".to_string(),
        true => "was called".to_string(),
    };
    println!("--load {}", load);
    let pos_args = parser.get_pos_args();
    if pos_args.len() != 0 {
        let mut c: u8 = 0;
        for i in pos_args {
            c += 1;
            println!("Positional arg {}: '{}'", c, i);
        }
    } else {
        println!("No positional arguments given.");
    }
}
