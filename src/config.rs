use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Switch {
        #[arg(required=true)]
        /// The filename of the theme (relative to theme dir)
        filename: String,
    }
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}
