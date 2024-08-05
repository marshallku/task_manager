use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Add,
    List,
    Update { id: u32 },
    Start { id: u32 },
    Pause { id: u32 },
    Done { id: u32 },
    Delete { id: u32 },
}
