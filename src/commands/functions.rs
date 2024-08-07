use chrono::{NaiveDate, Utc};
use std::error::Error;

use crate::{
    data::{status::TaskStatus, task::Task},
    utils::table::table_view,
};

const HOUR_IN_MINUTES: f32 = 60.0;

fn find_task_mut<'a>(tasks: &'a mut Vec<Task>, id: u32) -> Result<&'a mut Task, Box<dyn Error>> {
    tasks
        .iter_mut()
        .find(|task| task.id == id)
        .ok_or_else(|| "Task not found".into())
}

fn calculate_task_time(task: &Task) -> Result<f32, Box<dyn Error>> {
    let start_time = task.started_at.ok_or("No start time")?;
    let end_time = task.paused_at.or(task.completed_at).ok_or("No end time")?;
    Ok(end_time.signed_duration_since(start_time).num_minutes() as f32 / HOUR_IN_MINUTES)
}

pub fn add_task(
    tasks: &mut Vec<Task>,
    name: String,
    status: TaskStatus,
    deadline: NaiveDate,
    priority: String,
    estimated_hours: f32,
) -> Result<(), Box<dyn Error>> {
    let id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    let task = Task::new(id, name, status, deadline, priority, estimated_hours);
    tasks.push(task);
    Ok(())
}

pub fn list_tasks(tasks: &Vec<Task>) {
    table_view(&tasks)
}

pub fn start_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), Box<dyn Error>> {
    let task = find_task_mut(tasks, id)?;
    task.status = TaskStatus::InProgress;
    task.started_at = Some(Utc::now().naive_utc());
    Ok(())
}

pub fn pause_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), Box<dyn Error>> {
    let task = find_task_mut(tasks, id)?;
    task.status = TaskStatus::Paused;
    task.paused_at = Some(Utc::now().naive_utc());
    task.time = calculate_task_time(task)?;
    task.started_at = None;
    Ok(())
}

pub fn done_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), Box<dyn Error>> {
    let task = find_task_mut(tasks, id)?;
    task.status = TaskStatus::Done;
    task.completed_at = Some(Utc::now().naive_utc());
    task.time = calculate_task_time(task)?;
    task.started_at = None;
    task.paused_at = None;
    Ok(())
}

pub fn delete_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), Box<dyn Error>> {
    let pos = tasks
        .iter()
        .position(|task| task.id == id)
        .ok_or("Task not found")?;
    tasks.remove(pos);
    Ok(())
}
