use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_json_from_file<P: AsRef<Path>>(path: P) -> serde_json::Value {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("File open error, wrong path?");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let response: serde_json::Value = serde_json::from_reader(reader).expect("JSON Badly Formatted");

    // Return the `User`.
    response
}