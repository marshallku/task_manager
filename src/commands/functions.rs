use chrono::NaiveDate;

use crate::{data::task::Task, utils::table::table_view};

pub fn add_task(
    tasks: &mut Vec<Task>,
    name: String,
    status: String,
    deadline: String,
    priority: String,
    estimated_hours: f32,
) {
    let id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    let deadline_date =
        NaiveDate::parse_from_str(&deadline, "%Y-%m-%d").expect("Invalid date format");
    let task = Task::new(id, name, status, deadline_date, priority, estimated_hours);
    tasks.push(task);
    println!("Task added successfully.");
}

pub fn list_tasks(tasks: &Vec<Task>) {
    table_view(&tasks)
}

pub fn update_task(tasks: &mut Vec<Task>, id: u32, status: String, time: f32) {
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.status = status;
        task.time = time;
        println!("Task updated successfully.");
    } else {
        println!("Task not found.");
    }
}

pub fn delete_task(tasks: &mut Vec<Task>, id: u32) {
    if let Some(pos) = tasks.iter().position(|task| task.id == id) {
        tasks.remove(pos);
        println!("Task deleted successfully.");
    } else {
        println!("Task not found.");
    }
}
