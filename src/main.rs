use anyhow::Result;

use clap::Parser;
#[cfg(feature = "csv")]
use mkdata_cli::generator::csv::generate_csv;
use mkdata_cli::{
    cli::{Cli, GenerateSubcommands, MainCommands},
    error::Error::UnimplementedError,
};

fn main() -> Result<()> {
    let args = Cli::try_parse()?;
    match args {
        Cli {
            command: MainCommands::Generate { file_type },
        } => match file_type {
            #[cfg(feature = "csv")]
            GenerateSubcommands::CSV { path, rows, cols } => generate_csv(path, rows, cols),
            _ => Err(UnimplementedError { file_type }.into()),
        },
    }
}
