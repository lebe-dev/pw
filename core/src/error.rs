use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("io error")]
    IOError(#[from] io::Error),

    #[error("encryption error")]
    EncryptionError,

    #[error("decryption error")]
    DecryptionError
}