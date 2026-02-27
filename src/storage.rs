use crate::task::Task;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn get_file_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("No home dir");
    path.push(".rust_todo.json");
    path
}

pub fn load_tasks() -> Vec<Task> {
    let path = get_file_path();
    if !path.exists() {
        return Vec::new();
    }

    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".into());
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let path = get_file_path();
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())
}