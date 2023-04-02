use log::{debug, error};
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug)]
pub enum MarseError {
    FileError,
}

pub fn file_to_html(path: &Path) -> Result<String, MarseError> {
    debug!("Loading {path:#?}");

    let h1 = Regex::new("^#(.[a-zA-Z0-9])").unwrap();
    let h2 = Regex::new("^##(.[a-zA-Z0-9])").unwrap();
    let h3 = Regex::new("^###(.[a-zA-Z0-9])").unwrap();
    let h4 = Regex::new("^####(.[a-zA-Z0-9])").unwrap();
    let h5 = Regex::new("^#####(.[a-zA-Z0-9])").unwrap();
    let h6 = Regex::new("^######(.[a-zA-Z0-9])").unwrap();

    let file = match File::open(path) {
        Ok(v) => v,
        Err(e) => {
            error!("{e}");
            return Err(MarseError::FileError);
        }
    };

    let mut reader = BufReader::new(file);
    let mut file_contents = String::new();
    let mut html: Vec<String> = Vec::new();

    match reader.read_to_string(&mut file_contents) {
        Ok(v) => v,
        Err(e) => {
            error!("{e}");
            return Err(MarseError::FileError);
        }
    };

    for line in file_contents.lines() {
        if h1.is_match(line) {
            html.push(format!("<h1>{line}</h1>"));
        }
        if h2.is_match(line) {
            html.push(format!("<h2>{line}</h2>"));
        }
        if h3.is_match(line) {
            html.push(format!("<h3>{line}</h3>"));
        }
        if h4.is_match(line) {
            html.push(format!("<h4>{line}</h4>"));
        }
        if h5.is_match(line) {
            html.push(format!("<h5>{line}</h5>"));
        }
        if h6.is_match(line) {
            html.push(format!("<h6>{line}</h6>"));
        }
    }

    Ok(html.iter().map(|s| format!("{s}\n")).collect::<String>())
}
