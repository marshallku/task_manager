mod commands;
mod data;

use clap::Parser;

use commands::constants::Commands;

#[derive(Parser)]
#[command(name = "Task Manager")]
#[command(about = "A simple CLI task manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Add => {
            unimplemented!("Add task")
        }
        Commands::List => {
            unimplemented!("List tasks")
        }
        Commands::Update => {
            unimplemented!("Update task")
        }
        Commands::Delete => {
            unimplemented!("Delete task")
        }
    }
}
