mod commands;
mod data;
mod utils;

use clap::Parser;

use commands::{
    constants::Commands,
    functions::{add_task, delete_task, list_tasks, update_task},
};
use data::task::Task;
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
        Commands::Add {
            name,
            status,
            deadline,
            priority,
            estimated_hours,
        } => {
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
        Commands::Update { id, status, time } => {
            update_task(&mut tasks, id, status, time);
            save_tasks(&tasks).expect("Failed to save tasks");
        }
        Commands::Delete { id } => {
            delete_task(&mut tasks, id);
            save_tasks(&tasks).expect("Failed to save tasks");
        }
    }
}
