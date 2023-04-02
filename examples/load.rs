use marse::*;
use std::path::Path;

fn main() {
    println!(
        "{}",
        file_to_html(&Path::new(env!("CARGO_MANIFEST_DIR")).join(Path::new("examples/test.md")))
            .unwrap()
    );
}
