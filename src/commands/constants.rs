use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Add,
    List,
    Start { id: u32 },
    Pause { id: u32 },
    Done { id: u32 },
    Delete { id: u32 },
}
