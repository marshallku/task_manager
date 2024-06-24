use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Add,
    List,
    Update,
    Delete,
}
