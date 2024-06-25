use serde_json;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

use crate::data::task::Task;

pub fn get_file_path() -> PathBuf {
    let mut dir = dirs::config_dir().unwrap_or_else(|| dirs::home_dir().unwrap());
    dir.push(".task_manager");
    std::fs::create_dir_all(&dir).expect("Failed to create data directory");
    dir.push("tasks.json");
    dir
}

pub fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let file_path = get_file_path();
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &tasks)?;
    Ok(())
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let file_path = get_file_path();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}
