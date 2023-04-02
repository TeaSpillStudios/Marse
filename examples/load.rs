use marse::*;
use std::path::Path;

fn main() {
    pretty_env_logger::init_custom_env("MARSE_LOG");

    println!(
        "{}",
        file_to_html(&Path::new(env!("CARGO_MANIFEST_DIR")).join(Path::new("examples/test.md")))
            .unwrap()
    );
}
