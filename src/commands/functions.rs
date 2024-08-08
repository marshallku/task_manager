use chrono::{NaiveDate, Utc};

use crate::{
    data::{
        status::TaskStatus,
        task::{Task, TaskError},
    },
    utils::table::table_view,
};

const HOUR_IN_MINUTES: f32 = 60.0;

fn find_task_mut<'a>(tasks: &'a mut Vec<Task>, id: u32) -> Result<&'a mut Task, TaskError> {
    tasks
        .iter_mut()
        .find(|task| task.id == id)
        .ok_or(TaskError::NotFound)
}

pub fn calculate_task_time(task: &Task) -> Result<f32, TaskError> {
    if task.started_at.is_none() {
        return Ok(task.estimated_hours);
    }

    let start_time = task
        .started_at
        .ok_or_else(|| TaskError::InvalidState("No start time".to_string()))?;
    let end_time = task
        .paused_at
        .or(task.completed_at)
        .ok_or_else(|| TaskError::InvalidState("No end time".to_string()))?;

    Ok(end_time.signed_duration_since(start_time).num_minutes() as f32 / HOUR_IN_MINUTES)
}

pub fn add_task(
    tasks: &mut Vec<Task>,
    name: String,
    status: TaskStatus,
    deadline: NaiveDate,
    priority: String,
    estimated_hours: f32,
) -> Result<(), TaskError> {
    let id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    let task = Task::new(id, name, status, deadline, priority, estimated_hours);
    tasks.push(task);
    Ok(())
}

pub fn list_tasks(tasks: &Vec<Task>) {
    table_view(&tasks)
}

pub fn start_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), TaskError> {
    let task = find_task_mut(tasks, id)?;
    task.status = TaskStatus::InProgress;
    task.started_at = Some(Utc::now().naive_utc());
    Ok(())
}

pub fn pause_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), TaskError> {
    let task = find_task_mut(tasks, id)?;
    task.status = TaskStatus::Paused;
    task.paused_at = Some(Utc::now().naive_utc());
    task.time = calculate_task_time(task)?;
    task.started_at = None;
    Ok(())
}

pub fn done_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), TaskError> {
    let task = find_task_mut(tasks, id)?;
    task.status = TaskStatus::Done;
    task.completed_at = Some(Utc::now().naive_utc());
    task.time = calculate_task_time(task)?;
    task.started_at = None;
    task.paused_at = None;
    Ok(())
}

pub fn delete_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), TaskError> {
    let pos = tasks
        .iter()
        .position(|task| task.id == id)
        .ok_or(TaskError::NotFound)?;
    tasks.remove(pos);
    Ok(())
}
