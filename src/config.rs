use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    List,
    Switch {
        #[arg(required=true)]
        /// The theme name of the theme (relative to theme dir). Note: without .yml
        /// The file it looks is themename.yml
        theme_name: String,
    },
    Add {
        #[arg(required=true)]
        /// The theme name of the theme (relative to theme dir). Note: without .yml
        /// The file it looks is themename.yml
        theme_name: String,

        #[arg(required=true)]
        /// The path to theme file
        theme_path: String,

    },
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}
