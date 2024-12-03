use std::collections::HashMap;
use std::io::{BufRead, Cursor};

#[derive(Debug)]
pub struct TagParseError(pub usize, pub String);

pub fn parse(bytes: &[u8]) -> Result<HashMap<String, String>, TagParseError> {
    let mut result = HashMap::default();

    let mut buf = Cursor::new(bytes);

    let mut line = String::new();
    let mut line_number = 0;

    while let Ok(bytes_read) = buf.read_line(&mut line) {
        if bytes_read == 0 {
            // EOF
            break;
        }
        line_number += 1;
        let content = line.trim();
        if !content.is_empty() && !content.starts_with('#') {
            let mut split = content.split('=');
            let tag = split.next().ok_or_else(|| TagParseError(line_number, line.clone()))?;
            let value = split.next().ok_or_else(|| TagParseError(line_number, line.clone()))?;
            if split.next().is_some() {
                return Err(TagParseError(line_number, line.clone()));
            }
            result.insert(tag.to_string(), value.to_string());
        }
        line.clear();
    }

    Ok(result)
}
