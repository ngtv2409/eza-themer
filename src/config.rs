use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
#[command(version)]
pub enum Commands {
    /// List all .yml files in the theme directory
    List,
    /// Switch to a theme by name
    Switch {
        #[arg(required=true)]
        /// The theme name of the theme (relative to theme dir). Note: without .yml
        /// The file it looks is themename.yml
        theme_name: String,
    },
    /// Add a new theme from theme name and path to theme file
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
