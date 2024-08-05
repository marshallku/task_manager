mod commands;
mod data;
mod utils;

use clap::Parser;

use commands::{
    constants::Commands,
    functions::{
        add_task, delete_task, done_task, list_tasks, pause_task, start_task, update_task,
    },
};
use data::task::Task;
use utils::{
    input::get_input,
    storage::{load_tasks, save_tasks},
};

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
            let name = get_input("Enter task name: ");
            let status = get_input("Enter task status: ");
            let deadline = get_input("Enter task deadline (YYYY-MM-DD): ");
            let priority = get_input("Enter task priority: ");
            let estimated_hours = get_input("Enter task estimated hours: ")
                .parse::<f32>()
                .expect("Invalid estimated hours");

            add_task(
                &mut tasks,
                name,
                status,
                deadline,
                priority,
                estimated_hours,
            );
            save_tasks(&tasks).expect("Failed to save tasks");
        }
        Commands::List => list_tasks(&tasks),
        Commands::Update { id } => {
            let status = get_input("Enter task status: ");
            let time = get_input("Enter task time: ")
                .parse::<f32>()
                .expect("Invalid task time");

            update_task(&mut tasks, id, status, time);
            save_tasks(&tasks).expect("Failed to save tasks");
        }
        Commands::Start { id } => {
            start_task(&mut tasks, id);
            save_tasks(&tasks).expect("Failed to save tasks");
        }
        Commands::Pause { id } => {
            pause_task(&mut tasks, id);
            save_tasks(&tasks).expect("Failed to save tasks");
        }
        Commands::Done { id } => {
            done_task(&mut tasks, id);
            save_tasks(&tasks).expect("Failed to save tasks");
        }
        Commands::Delete { id } => {
            delete_task(&mut tasks, id);
            save_tasks(&tasks).expect("Failed to save tasks");
        }
    }
}
