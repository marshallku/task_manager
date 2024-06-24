use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Add {
        name: String,
        status: String,
        deadline: String,
        priority: String,
        estimated_hours: f32,
    },
    List,
    Update {
        id: u32,
        status: String,
        time: f32,
    },
    Delete {
        id: u32,
    },
}
