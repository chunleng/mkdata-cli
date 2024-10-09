use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: MainCommands,
}

#[derive(Subcommand)]
pub enum MainCommands {
    #[command(about = "Generate data of various file type")]
    Generate {
        #[command(subcommand)]
        file_type: GenerateSubcommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum GenerateSubcommands {
    #[command(about = "Generate CSV data (TODO)")]
    CSV,
    #[command(about = "Generate JSON data (TODO)")]
    JSON,
    #[command(about = "Generate PARQUET data (TODO)")]
    PARQUET,
    #[command(about = "Generate ORC data (TODO)")]
    ORC,
}
