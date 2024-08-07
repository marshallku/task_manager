mod commands;
mod data;
mod utils;

use core::panic;

use chrono::{NaiveDate, Utc};
use clap::Parser;

use commands::{
    constants::Commands,
    functions::{add_task, delete_task, done_task, list_tasks, pause_task, start_task},
};
use data::{status::TaskStatus, task::Task};
use inquire::{CustomType, DateSelect, Select, Text};
use utils::storage::{load_tasks, save_tasks};

#[derive(Parser)]
#[command(name = "Task Manager")]
#[command(about = "A simple CLI task manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();
    let mut tasks: Vec<Task> = load_tasks().unwrap_or_else(|_| Vec::new());

    match args.command {
        Commands::Add => {
            let name = match Text::new("Enter task name: ").prompt() {
                Ok(name) => name,
                Err(_) => panic!("Failed to get task name"),
            };
            let deadline = match DateSelect::new("Enter task deadline (YYYY-MM-DD): ")
                .with_min_date(NaiveDate::from(Utc::now().naive_utc()))
                .prompt()
            {
                Ok(deadline) => deadline,
                Err(_) => panic!("Failed to get task deadline"),
            };
            let priority = match Select::new("Enter task priority: ", vec!["Low", "Medium", "High"])
                .prompt()
            {
                Ok(priority) => priority.to_string(),
                Err(_) => panic!("Failed to get task priority"),
            };
            let estimated_hours: f32 = match CustomType::<f32>::new("Enter task estimated hours: ")
                .with_formatter(&|i| format!("{:.2}h", i))
                .with_error_message("Please type a valid number")
                .prompt()
            {
                Ok(estimated_hours) => estimated_hours,
                Err(_) => panic!("Failed to get task estimated hours"),
            };

            let status = TaskStatus::Todo;

            add_task(
                &mut tasks,
                name,
                status,
                deadline,
                priority,
                estimated_hours,
            )
            .unwrap_or_else(|err| panic!("Failed to add task: {}", err));
            save_tasks(&tasks).expect("Failed to save tasks");
        }
        Commands::List => list_tasks(&tasks),
        Commands::Start { id } => {
            start_task(&mut tasks, id)
                .unwrap_or_else(|err| panic!("Failed to start task: {}", err));
            save_tasks(&tasks).expect("Failed to save tasks");
        }
        Commands::Pause { id } => {
            pause_task(&mut tasks, id)
                .unwrap_or_else(|err| panic!("Failed to pause task: {}", err));
            save_tasks(&tasks).expect("Failed to save tasks");
        }
        Commands::Done { id } => {
            done_task(&mut tasks, id)
                .unwrap_or_else(|err| panic!("Failed to complete task: {}", err));
            save_tasks(&tasks).expect("Failed to save tasks");
        }
        Commands::Delete { id } => {
            delete_task(&mut tasks, id)
                .unwrap_or_else(|err| panic!("Failed to delete task: {}", err));
            save_tasks(&tasks).expect("Failed to save tasks");
        }
    }
}
