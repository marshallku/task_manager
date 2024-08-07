use std::error::Error;

use chrono::{NaiveDate, Utc};

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
    task.time = task
        .completed_at
        .unwrap()
        .signed_duration_since(task.started_at.unwrap())
        .num_minutes() as f32
        / HOUR_IN_MINUTES;
    task.started_at = None;
    Ok(())
}

pub fn done_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), Box<dyn Error>> {
    let task = find_task_mut(tasks, id)?;
    task.status = TaskStatus::Done;
    task.completed_at = Some(Utc::now().naive_utc());

    // Calculate time spent on task
    // If task is paused, calculate time spent until it was paused
    if task.paused_at.is_some() && task.started_at.is_some() {
        task.time = task
            .completed_at
            .unwrap()
            .signed_duration_since(task.started_at.unwrap())
            .num_minutes() as f32
            / HOUR_IN_MINUTES;
    // If task is not paused, calculate time spent until it was completed
    } else if task.started_at.is_some() {
        task.time = task
            .completed_at
            .unwrap()
            .signed_duration_since(task.started_at.unwrap())
            .num_minutes() as f32
            / HOUR_IN_MINUTES;
    } else {
        task.time = task.estimated_hours;
    }

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
