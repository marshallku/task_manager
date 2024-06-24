mod commands;
mod data;

use clap::Parser;

use commands::{
    constants::Commands,
    functions::{add_task, delete_task, list_tasks, update_task},
};
use data::task::Task;

#[derive(Parser)]
#[command(name = "Task Manager")]
#[command(about = "A simple CLI task manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();
    let mut tasks: Vec<Task> = Vec::new();

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
        }
        Commands::List => list_tasks(&tasks),
        Commands::Update { id, status, time } => {
            update_task(&mut tasks, id, status, time);
        }
        Commands::Delete { id } => {
            delete_task(&mut tasks, id);
        }
    }
}
