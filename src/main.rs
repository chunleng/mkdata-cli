use anyhow::Result;

use clap::Parser;
use mkdata_cli::{
    cli::{Cli, MainCommands},
    error::Error::UnimplementedError,
};

fn main() -> Result<()> {
    let args = Cli::try_parse()?;
    match args {
        Cli {
            command: MainCommands::Generate { file_type },
        } => Err(UnimplementedError { file_type }.into()),
    }
}
