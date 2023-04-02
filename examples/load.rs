use marse::*;
use std::path::Path;

fn main() {
    println!("{}", file_to_html(Path::new("./test.md")).unwrap());
}
