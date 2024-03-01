use crate::models;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Args)]
pub struct RequestParams {}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short)]
        method: models::Method,
        #[arg(short)]
        url: String,
        #[arg(short, long)]
        name: String,
    },
    List,
    Delete {
        id: i64,
    },
    Edit {
        id: i64,
        #[arg(short)]
        method: Option<models::Method>,
        #[arg(short)]
        url: Option<String>,
        #[arg(short, long)]
        name: Option<String>,
    },
    Execute {
        id: i64,
    },
}
