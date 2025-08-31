use super::*;

#[test]
fn base() {
    let mut argparse = Parser::new("test");
    argparse.add_argument("--test", Some("-t"), "This is just a test", None);
}
