use log::error;
use std::{fs::read_to_string, path::Path};

pub enum MarseError {
    FileDoesNotExist,
}

pub fn file_to_html(path: &Path) -> Result<String, MarseError> {
    let markdown = match read_to_string(path) {
        Ok(v) => v,
        Err(e) => {
            error!("{e}");
            return Err(MarseError::FileDoesNotExist);
        }
    };

    Ok(markdown)
}
