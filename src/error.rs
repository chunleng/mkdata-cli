use thiserror::Error;

use crate::cli::GenerateSubcommands;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{:?} is not yet implemented", .file_type)]
    UnimplementedError { file_type: GenerateSubcommands },
}
