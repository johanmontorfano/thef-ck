use std::{fs, io};

// This function takes a path as an argument, and returns a `Result` which tells
// if the content at the given path as been read successfully or not with it's
// content. It returns raw content.
pub fn read_file(path: &String) -> Result<String, io::Error> {
    return fs::read_to_string(path);
}