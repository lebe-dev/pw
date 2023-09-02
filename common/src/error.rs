use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("io error")]
    IOError(#[from] io::Error),

    #[error("decryption error")]
    DecryptionError
}