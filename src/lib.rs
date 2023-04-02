use log::{debug, error};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug)]
pub enum MarseError {
    FileError,
}

pub fn file_to_html(path: &Path) -> Result<String, MarseError> {
    debug!("Loading {path:#?}");

    let file = match File::open(path) {
        Ok(v) => v,
        Err(e) => {
            error!("{e}");
            return Err(MarseError::FileError);
        }
    };

    let mut reader = BufReader::new(file);
    let mut file_contents = String::new();

    match reader.read_to_string(&mut file_contents) {
        Ok(v) => v,
        Err(e) => {
            error!("{e}");
            return Err(MarseError::FileError);
        }
    };

    Ok(file_contents)
}
