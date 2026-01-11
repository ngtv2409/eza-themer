use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
#[command(version)]
pub enum Commands {
    /// List all .yml files in the theme directory
    List,
    /// Switch to a theme by name
    Switch {
        #[arg(required_unless_present = "interactive")]
        /// The theme name of the theme (relative to theme dir). Note: without .yml
        /// The file it looks is themename.yml
        theme_name: Option<String>,

        #[arg(short, long)]
        /// Interactive mode
        interactive: bool,
    },
    /// Add a new theme from theme name and path to theme file
    Add {
        #[arg(required = true)]
        /// The theme name of the theme (relative to theme dir). Note: without .yml
        /// The file it looks is themename.yml
        theme_name: String,

        #[arg(required = true)]
        /// The path to theme file
        theme_path: String,
    },
    /// Preview a theme by running eza on a test dir
    Preview {
        #[arg(required_unless_present = "interactive")]
        /// The theme name of the theme (relative to theme dir). Note: without .yml
        /// The file it looks is themename.yml
        theme_name: Option<String>,

        #[arg(short, long)]
        /// Interactive mode
        interactive: bool,
    },
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}
