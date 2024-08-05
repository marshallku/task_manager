use chrono::{NaiveDate, Utc};

use crate::{data::task::Task, utils::table::table_view};

const MINUTES_IN_HOUR: f32 = 60.0;

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

pub fn start_task(tasks: &mut Vec<Task>, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.status = "Doing".to_string();
        task.started_at = Some(Utc::now().naive_utc());
        println!("Task started successfully.");
    } else {
        println!("Task not found.");
    }
}

pub fn pause_task(tasks: &mut Vec<Task>, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.paused_at = Some(Utc::now().naive_utc());
        task.time = task
            .started_at
            .unwrap()
            .signed_duration_since(task.paused_at.unwrap())
            .num_minutes() as f32
            * MINUTES_IN_HOUR;
        task.started_at = None;
        println!("Task paused successfully.");
    } else {
        println!("Task not found.");
    }
}

pub fn done_task(tasks: &mut Vec<Task>, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.status = "Done".to_string();
        task.completed_at = Some(Utc::now().naive_utc());

        // If task is not paused, calculate time.
        if task.paused_at.is_none() {
            task.time = task
                .started_at
                .unwrap()
                .signed_duration_since(task.completed_at.unwrap())
                .num_minutes() as f32
                * MINUTES_IN_HOUR;
        }

        task.started_at = None;
        task.paused_at = None;
        println!("Task completed successfully.");
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
