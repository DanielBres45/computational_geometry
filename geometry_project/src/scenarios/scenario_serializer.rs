use serde::{Deserialize, Serialize};

use std::{
    fs::{self, OpenOptions},
    io::Write,
};

pub fn save_entities_to_file<T: Serialize>(entities: &Vec<T>, file_name: &str, append: bool) {
    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(append)
        .open(file_name)
    {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {}, error {}", file_name, e),
    };

    let serialized = match serde_json::to_string(&entities) {
        Ok(s) => s,
        Err(e) => panic!("failed to serialize entities, {}", e),
    };

    match file.write(serialized.as_bytes()) {
        Err(e) => panic!("Failed to write file, {}", e),
        _ => (),
    };
}

pub fn deserialize_from_file<T>(file_name: &str) -> Result<Vec<T>, &'static str>
where
    T: serde::de::DeserializeOwned,
{
    // Read the file contents
    let contents = fs::read_to_string(file_name).map_err(|_| "Failed to read file")?;

    // Deserialize the JSON content
    let items: Vec<T> =
        serde_json::from_str(&contents).map_err(|_| "Failed to deserialize JSON")?;

    Ok(items)
}
