use crate::models;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        method: models::Method,
        url: String,
        #[arg(short, long)]
        name: String,
    },
    List,
    Delete {
        id: i64,
    },
    Execute {
        id: i64,
    },
}
