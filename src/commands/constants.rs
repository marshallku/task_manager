use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Add,
    List,
    Update { id: u32 },
    Delete { id: u32 },
}
