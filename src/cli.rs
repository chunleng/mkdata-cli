use std::path::Path;

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
    #[cfg(feature = "csv")]
    #[command(about = "Generate CSV data")]
    CSV {
        #[arg(
            short,
            long,
            default_value = "./output.csv",
            help = "Number of rows to generate"
        )]
        path: Box<Path>,

        #[arg(
            short,
            long,
            default_value_t = 10,
            value_name = "NO_OF_ROWS",
            help = "Number of rows to generate"
        )]
        rows: usize,

        #[arg(
            short,
            long = "col",
            required = true,
            value_name = "COLUMN_NAME",
            value_delimiter = ',',
            help = "List of column names"
        )]
        cols: Vec<String>,
    },
    #[command(about = "Generate JSON data (TODO)")]
    JSON,
    #[command(about = "Generate PARQUET data (TODO)")]
    PARQUET,
    #[command(about = "Generate ORC data (TODO)")]
    ORC,
}
