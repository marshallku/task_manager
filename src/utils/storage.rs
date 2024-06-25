use serde_json;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

use crate::data::task::Task;

const FILENAME: &str = "tasks.json";

pub fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let file = File::create(FILENAME)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &tasks)?;
    Ok(())
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}
